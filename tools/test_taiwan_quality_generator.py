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

    def test_bare_word_cannot_be_disguised_as_whitelist(self):
        self.rules['guarded'][0]['allow'].append(['質量', '品質'])
        with self.assertRaisesRegex(ValueError, 'longer context'):
            validate_rules(self.rules)


if __name__ == '__main__':
    unittest.main()
