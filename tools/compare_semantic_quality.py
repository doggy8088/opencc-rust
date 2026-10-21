#!/usr/bin/env python3
"""Compare the captured predecessor and current ports on constructed semantic cases."""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'opencc-py/src'))
from opencc_py import converter


def main():
    directory = ROOT / 'opencc-rust/quality'
    cases = json.loads((directory / 'semantic-cases.json').read_text())
    before = json.loads((directory / 'semantic-before.json').read_text())
    rules = json.loads((directory / 'taiwan-rules.json').read_text())
    if before['sources'] != [case['source'] for case in cases]:
        raise ValueError('Comparison sources changed; a new baseline capture is required')
    report = {
        'scope': 'Constructed contrastive tests only; not a population accuracy estimate.',
        'case_count': len(cases),
        'guarded_word_count': len(rules['guarded']),
        'explicit_whitelist_pairs': sum(len(group['allow']) for group in rules['guarded']),
        'locales': {},
    }
    failures = []
    for locale in ('tw2', 'twp'):
        c = converter('cn', locale)
        counts = {}
        categories = {}
        for index, case in enumerate(cases):
            expected = case.get('expected_twp', case['expected']) if locale == 'twp' else case['expected']
            current = c(case['source'])
            old = before['outputs'][locale][index]
            bucket = counts.setdefault(case['kind'], dict(total=0, before_correct=0, after_correct=0))
            bucket['total'] += 1
            bucket['before_correct'] += int(old == expected)
            bucket['after_correct'] += int(current == expected)
            category = categories.setdefault(case['category'], dict(total=0, before_errors=0, after_errors=0))
            category['total'] += 1
            category['before_errors'] += int(old != expected)
            category['after_errors'] += int(current != expected)
            if current != expected or c(current) != current:
                failures.append(dict(locale=locale, source=case['source'], expected=expected, actual=current))
        report['locales'][locale] = dict(by_kind=counts, by_category=categories)
    report['failures'] = failures
    (directory / 'semantic-comparison.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps({k: v for k, v in report.items() if k != 'locales'}, ensure_ascii=False))
    for locale, result in report['locales'].items():
        print(locale, json.dumps(result['by_kind']))
    if failures:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
