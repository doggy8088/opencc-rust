#!/usr/bin/env python3
"""Generate identical Taiwan quality patches for every standalone implementation."""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def validate_rules(rules):
    """Reject unsafe fallbacks and conflicting entries before emitting any files."""
    guarded = {group['source'] for group in rules['guarded']}
    singles = {'賬': '帳', '羣': '群'}
    seen = {}
    for key, value in rules['words']:
        if key in guarded:
            raise ValueError(f'Guarded word cannot have a global replacement: {key}')
        if len(key) == 1 and singles.get(key) != value:
            raise ValueError(f'Unreviewed single-character replacement: {key}')
    for group in rules['guarded']:
        for key, value in group['allow']:
            if group['source'] not in key or len(key) <= len(group['source']):
                raise ValueError(f'Whitelist needs an explicit longer context: {key}')
    pairs = list(rules['words']) + list(rules['phrases'])
    pairs += [(key, key) for key in rules['protected']]
    pairs += [pair for group in rules['guarded'] for pair in group['allow']]
    for key, value in pairs:
        if key in guarded and key != value:
            raise ValueError(f'Guarded word cannot have a global replacement: {key}')
        if len(key) == 1 and singles.get(key) != value:
            raise ValueError(f'Unreviewed single-character replacement: {key}')
        if key in seen and seen[key] != value:
            raise ValueError(f'Conflicting targets for {key}: {seen[key]} / {value}')
        seen[key] = value
    validate_source_cn(rules['source_cn'])


def validate_source_cn(config):
    """Source patches must stay narrow: explicit context around one reviewed character."""
    char = config['char']
    if len(char) != 1:
        raise ValueError(f'Source patch needs exactly one character: {char}')
    for tail in config['guard_tails']:
        if len(tail) != 1:
            raise ValueError(f'Guard tail must be one character: {tail}')
    for prefix in config['guard_prefixes']:
        if not prefix or char in prefix:
            raise ValueError(f'Guard prefix must precede {char}: {prefix}')
    seen = {}
    for key, value in config['phrases']:
        if len(key) < 2 or char not in key:
            raise ValueError(f'Source phrase needs {char} with explicit context: {key}')
        if len(key) != len(value):
            raise ValueError(f'Source phrase must convert character by character: {key}')
        if key in seen and seen[key] != value:
            raise ValueError(f'Conflicting targets for {key}: {seen[key]} / {value}')
        seen[key] = value


def source_cn_patch(config, constants):
    """Patch the shared cn source group for characters valid in both scripts.

    干 is both a Simplified and a Traditional character, but STCharacters maps
    it to 幹. Without these entries, already-Traditional input such as 干擾
    misses the Simplified-only key 干扰, and keys ending in 干 (府干, 被干)
    swallow the 干 of a following 干擾/干預/干涉.
    """
    char = config['char']
    first = lambda value: value.split(' ')[0]
    upstream = {key: first(value) for key, value in entries(constants['STPhrases'])}
    s2t = {key: first(value) for key, value in entries(constants['STCharacters'])}
    s2t.update(upstream)
    variants = dict(entries(constants['TWVariants']))
    patch = {}
    # Upstream verb keys such as 府干/被干 swallow the 干 of 干擾/干預/干涉.
    # Only reviewed adverb/function-word prefixes are guarded: after a noun
    # (餅干, 骨干) the tail may start another word, e.g. 餅乾預計.
    for prefix in config['guard_prefixes']:
        key = prefix + char
        value = upstream.get(key, '')
        if len(value) != len(key) or value.endswith(char):
            raise ValueError(f'Guard prefix needs an upstream {char} key: {key}')
        for tail in config['guard_tails']:
            if key + tail not in upstream:
                patch[key + tail] = value[:-1] + char + replace(tail, s2t)
    patch.update(config['phrases'])
    # Traditional input is already final: keep every Traditional form as-is
    # (輪流干擾 would otherwise hit the upstream key 流干).
    for key, value in list(upstream.items()) + list(patch.items()):
        if char in value and value != key:
            for form in (value, replace(value, variants)):
                if form not in upstream:
                    patch.setdefault(form, form)
    convert = {**s2t, **patch}
    for key, value in patch.items():
        if replace(key, convert) != value or replace(value, convert) != value:
            raise ValueError(f'Source patch is not stable: {key} -> {value}')
    return patch


def entries(text):
    for line in re.split(r'[|\n]', text):
        line = line.strip()
        if not line:
            continue
        pair = line.split('\t', 1) if '\t' in line else line.split(' ', 1)
        if len(pair) == 2:
            yield tuple(pair)


def replace(text, rules):
    # Longest prefix, one pass: replacements are never recursively processed.
    result = []
    i = 0
    by_first = {}
    for key in sorted(rules, key=len, reverse=True):
        by_first.setdefault(key[0], []).append(key)
    while i < len(text):
        for key in by_first.get(text[i], []):
            if text.startswith(key, i):
                result.append(rules[key])
                i += len(key)
                break
        else:
            result.append(text[i])
            i += 1
    return ''.join(result)


def main():
    rules = json.loads((ROOT / 'opencc-rust/quality/taiwan-rules.json').read_text())
    validate_rules(rules)
    guarded = {group['source'] for group in rules['guarded']}
    normal = dict(rules['words'])
    normal.update((word, word) for word in guarded)
    normal.update((word, word) for word in rules['protected'])
    phrases = dict(normal)
    for head, tails in rules['through'].items():
        for tail in tails:
            phrases['通過' + tail] = head + replace(tail, normal)
    for tail in rules['visit']:
        phrases['訪問' + tail] = '造訪' + replace(tail, normal)
    for context in rules['pricing']:
        phrases[context + '套餐'] = replace(context, normal) + '方案'
        phrases['套餐' + context] = '方案' + replace(context, normal)
    # Longer noun exceptions beat save actions; unknown ambiguous words are preserved.
    phrases['通過'] = '通過'
    phrases['套餐'] = '套餐'
    for noun in rules['save_nouns']:
        phrases['保存' + noun] = '保存' + noun
    for prefix in rules['save_prefixes']:
        phrases[prefix + '保存'] = prefix + '保存'
    for source, target in rules['phrases']:
        phrases[source] = target
    for group in rules['guarded']:
        phrases.update(group['allow'])
    for tool in rules['approval_tools']:
        for check in rules['approval_checks']:
            phrases['通過' + tool + check] = '通過' + replace(tool + check, normal)

    source = (ROOT / 'OpenCC/src/OpenCC/Internal/DictData.cs').read_text(encoding='utf-8-sig')
    constants = dict(re.findall(r'public const string (\w+) = """(.*?)""";', source, re.S))
    # Include every implementation's existing custom additions in the audit.
    additions = (ROOT / 'opencc-py/src/opencc_py/locale_data.py').read_text()
    additions = re.search(r'TW_PHRASES_CUSTOM_ADDITIONS = """(.*?)"""', additions, re.S)[1]
    cn_patch = source_cn_patch(rules['source_cn'], constants)
    patches = {'cn': '\n'.join(k + '\t' + v for k, v in sorted(cn_patch.items())) + '\n'}
    audit = {}
    disabled = {}
    for locale, names in {
        'tw2': ['TWVariants', 'TWPhrasesCustom', 'TW2Tai'],
        'twp': ['TWVariants', 'TWPhrasesIT', 'TWPhrasesName', 'TWPhrasesOther'],
    }.items():
        base = dict(e for name in names for e in entries(constants[name]))
        if locale == 'tw2':
            base.update(entries(additions))
        disabled[locale] = {}
        safe_base = {key: value for key, value in base.items()
                     if not any(word in key for word in guarded)}
        for key, value in list(base.items()):
            if (key not in phrases and key not in rules['retain_upstream']
                    and any(word in key for word in guarded)):
                safe = replace(key, {**safe_base, **normal})
                if safe != value:
                    disabled[locale][key] = {'before': value, 'after': safe}
                    base[key] = safe
        patch = {}
        patch.update((key, item['after']) for key, item in disabled[locale].items())
        # Repair targets of longer dictionary entries too, so a compound cannot
        # bypass a single-word rule or emit an obsolete character variant.
        for key, value in base.items():
            updated = replace(value, phrases)
            if updated != value:
                patch[key] = updated
        audit[locale] = {key: value for key, value in patch.items()
                         if key not in disabled[locale]}
        # Newly composed phrases must also convert their other words (e.g.
        # 軟件項目 -> 軟體專案), without recursively converting base targets.
        canonical = {**base, **phrases}
        patch.update((key, replace(value, canonical)) for key, value in phrases.items())
        patches[locale] = '\n'.join(k + '\t' + v for k, v in sorted(patch.items())) + '\n'

    def write(path, text):
        destination = ROOT / path
        if '--check' in sys.argv:
            if not destination.exists() or destination.read_text() != text:
                raise SystemExit(f'Stale generated file: {path}')
        else:
            destination.write_text(text)

    semantic = json.loads((ROOT / 'opencc-rust/quality/semantic-cases.json').read_text())
    fixtures = (ROOT / 'opencc-rust/quality/cases.tsv').read_text()
    for case in semantic:
        if 'expected_twp' in case:
            fixtures += 'tw2\t' + case['source'] + '\t' + case['expected'] + '\n'
            fixtures += 'twp\t' + case['source'] + '\t' + case['expected_twp'] + '\n'
        else:
            fixtures += 'both\t' + case['source'] + '\t' + case['expected'] + '\n'
    for directory in ['opencc-rust/tests', 'opencc-cli/tests', 'opencc-js/test',
                      'opencc-py/tests', 'opencc-swift/Tests/OpenCCSwiftTests',
                      'OpenCC/tests/OpenCC.Tests']:
        write(directory + '/taiwan-quality.tsv', fixtures)

    header = '// Generated by opencc-rust/tools/generate_taiwan_quality.py; edit opencc-rust/quality/taiwan-rules.json.\n'
    write('opencc-rust/src/taiwan_quality.rs', header + '\n'.join(f'pub const {loc.upper()}: &str = r####"{value}"####;\n' for loc, value in patches.items()))
    write('opencc-py/src/opencc_py/taiwan_quality.py', '# Generated from opencc-rust/quality/taiwan-rules.json.\n' + '\n'.join(f'{loc.upper()} = """{value}"""\n' for loc, value in patches.items()))
    write('opencc-swift/Sources/OpenCCSwift/TaiwanQuality.swift', header + 'enum TaiwanQuality {\n' + '\n'.join(f'    static let {loc} = """\n{value}"""\n' for loc, value in patches.items()) + '}\n')
    write('OpenCC/src/OpenCC/Internal/TaiwanQuality.cs', header + 'namespace OpenCC.Internal;\n\ninternal static class TaiwanQuality\n{\n' + '\n'.join(f'    internal const string {loc.upper()} = @"{value}";\n' for loc, value in patches.items()) + '}')
    write('opencc-js/src/custom/TaiwanQuality.js', header + '\n'.join(f'export const {loc} = {json.dumps(list(entries(value)), ensure_ascii=False)};\n' for loc, value in patches.items()))
    write('opencc-rust/quality/compound-repairs.json', json.dumps(audit, ensure_ascii=False, indent=2) + '\n')
    write('opencc-rust/quality/disabled-upstream.json', json.dumps(disabled, ensure_ascii=False, indent=2) + '\n')


if __name__ == '__main__':
    main()
