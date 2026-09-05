use std::collections::HashMap;
use compact_str::CompactString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemRule {
    Include,
    Exclude,
}

#[derive(Debug, Clone, Default)]
pub struct TrieNode {
    rule: Option<FilesystemRule>,
    children: HashMap<String, TrieNode>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct RuleTrie {
    root: TrieNode,
}

impl RuleTrie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_ignores(ignores: &[CompactString]) -> Self {
        let mut trie = Self::new();
        for path in ignores {
            trie.insert(path, FilesystemRule::Exclude);
        }
        trie
    }

    pub fn insert(&mut self, path: &str, rule: FilesystemRule) {
        let mut node = &mut self.root;
        for seg in path.split('/').filter(|s| !s.is_empty()) {
            node = node.children.entry(seg.to_string()).or_default();
        }
        node.rule = Some(rule);
    }

    pub fn query(&self, path: &str) -> Option<FilesystemRule> {
        let mut node = &self.root;
        let mut best = None;
        if let Some(rule) = node.rule {
            best = Some(rule);
        }
        for seg in path.split('/').filter(|s| !s.is_empty()) {
            if let Some(child) = node.children.get(seg) {
                node = child;
                if let Some(rule) = node.rule {
                    best = Some(rule);
                }
            } else {
                break;
            }
        }
        best
    }

    /// Collect all rules for building ignore overrides.
    pub fn iter_rules(&self) -> Vec<(String, FilesystemRule)> {
        let mut result = Vec::new();
        self.collect_rules(&self.root, String::new(), &mut result);
        result
    }

    fn collect_rules(
        &self,
        node: &TrieNode,
        current_path: String,
        result: &mut Vec<(String, FilesystemRule)>,
    ) {
        if let Some(rule) = node.rule {
            result.push((current_path.clone(), rule));
        }
        for (seg, child) in &node.children {
            let mut path = current_path.clone();
            if !path.is_empty() {
                path.push('/');
            }
            path.push_str(seg);
            self.collect_rules(child, path, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exclude_path_matches_children() {
        let mut rules = RuleTrie::new();
        rules.insert("/foo", FilesystemRule::Exclude);

        assert_eq!(rules.query("/foo"), Some(FilesystemRule::Exclude));
        assert_eq!(rules.query("/foo/bar"), Some(FilesystemRule::Exclude));
        assert_eq!(rules.query("/foo/bar/baz"), Some(FilesystemRule::Exclude));

        assert_eq!(rules.query("/foobar"), None);
        assert_eq!(rules.query("/bar"), None);
    }

    #[test]
    fn more_specific_rule_wins() {
        let mut rules = RuleTrie::new();

        rules.insert("/foo", FilesystemRule::Exclude);
        rules.insert("/foo/bar", FilesystemRule::Include);

        assert_eq!(rules.query("/foo"), Some(FilesystemRule::Exclude));
        assert_eq!(rules.query("/foo/bar"), Some(FilesystemRule::Include));
        assert_eq!(rules.query("/foo/bar/baz"), Some(FilesystemRule::Include));
        assert_eq!(rules.query("/foo/baz"), Some(FilesystemRule::Exclude));
    }

    #[test]
    fn include_then_exclude() {
        let mut rules = RuleTrie::new();

        rules.insert("/foo", FilesystemRule::Include);
        rules.insert("/foo/bar", FilesystemRule::Exclude);

        assert_eq!(rules.query("/foo"), Some(FilesystemRule::Include));
        assert_eq!(rules.query("/foo/bar"), Some(FilesystemRule::Exclude));
        assert_eq!(rules.query("/foo/bar/baz"), Some(FilesystemRule::Exclude));
        assert_eq!(rules.query("/foo/baz"), Some(FilesystemRule::Include));
    }
}
