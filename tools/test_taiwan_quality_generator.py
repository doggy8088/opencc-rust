"""Policy regressions: unsafe fallbacks must fail before they reach any port."""
import copy
import json
import unittest
from pathlib import Path
from generate_taiwan_quality import validate_rules


class PolicyValidationTests(unittest.TestCase):
    def setUp(self):
        self.rules = json.loads((Path(__file__).resolve().parents[1] / 'quality/taiwan-rules.json').read_text())

    def test_reviewed_policy_is_valid(self):
        validate_rules(self.rules)

    def test_guarded_fallback_is_rejected_in_both_rule_sections(self):
        for section in ('words', 'phrases'):
            with self.subTest(section=section):
                rules = copy.deepcopy(self.rules)
                rules[section].append(['質量', '品質'])
                with self.assertRaisesRegex(ValueError, 'Guarded word'):
                    validate_rules(rules)

    def test_single_character_expansion_is_rejected(self):
        for section in ('words', 'phrases'):
            with self.subTest(section=section):
                rules = copy.deepcopy(self.rules)
                rules[section].append(['雲', '雲端'])
                with self.assertRaisesRegex(ValueError, 'single-character'):
                    validate_rules(rules)

    def test_conflicting_whitelists_are_rejected(self):
        self.rules['phrases'].append(['影片質量', '影片重量'])
        with self.assertRaisesRegex(ValueError, 'Conflicting targets'):
            validate_rules(self.rules)

    def test_source_phrase_needs_context_around_the_character(self):
        for pair in (['干', '幹'], ['餅乾', '餅乾']):
            with self.subTest(pair=pair):
                rules = copy.deepcopy(self.rules)
                rules['source_cn']['phrases'].append(pair)
                with self.assertRaisesRegex(ValueError, 'explicit context'):
                    validate_rules(rules)

    def test_source_phrase_must_keep_length(self):
        self.rules['source_cn']['phrases'].append(['干拌面', '乾拌麵條'])
        with self.assertRaisesRegex(ValueError, 'character by character'):
            validate_rules(self.rules)

    def test_additional_source_group_is_valid(self):
        self.assertTrue(self.rules.get('source_cn_groups'))
        validate_rules(self.rules)

    def test_additional_source_phrase_needs_its_own_character(self):
        rules = copy.deepcopy(self.rules)
        rules['source_cn_groups'][0]['phrases'].append(['血壓', '血壓'])
        with self.assertRaisesRegex(ValueError, 'explicit context'):
            validate_rules(rules)

    def test_source_guard_prefix_cannot_contain_the_character(self):
        self.rules['source_cn']['guard_prefixes'].append('干')
        with self.assertRaisesRegex(ValueError, 'Guard prefix'):
            validate_rules(self.rules)

    def test_bare_word_cannot_be_disguised_as_whitelist(self):
        self.rules['guarded'][0]['allow'].append(['質量', '品質'])
        with self.assertRaisesRegex(ValueError, 'longer context'):
            validate_rules(self.rules)


if __name__ == '__main__':
    unittest.main()
