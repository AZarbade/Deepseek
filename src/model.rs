use super::lexer::Lexer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

type TermFreq = HashMap<String, usize>;
type DocFreq = HashMap<String, usize>;

#[derive(Deserialize, Serialize)]
pub struct Doc {
    tf: TermFreq,
    count: usize,
    last_modified: SystemTime,
}

type Docs = HashMap<PathBuf, Doc>;

#[derive(Default, Deserialize, Serialize)]
pub struct Model {
    pub docs: Docs,
    df: DocFreq,
}

impl Model {
    fn remove_document(&mut self, file_path: &Path) {
        if let Some(doc) = self.docs.remove(file_path) {
            for term in doc.tf.keys() {
                if let Some(freq) = self.df.get_mut(term) {
                    *freq -= 1;
                }
            }
        }
    }

    pub fn requires_reindexing(&mut self, file_path: &Path, last_modified: SystemTime) -> bool {
        if let Some(doc) = self.docs.get(file_path) {
            return doc.last_modified < last_modified;
        }
        true
    }

    pub fn search_query(&self, query: &[char]) -> Vec<(PathBuf, f32)> {
        let mut result = Vec::new();
        let tokens = Lexer::new(&query).collect::<Vec<_>>();
        for (path, doc) in &self.docs {
            let mut rank = 0f32;
            for token in &tokens {
                rank += compute_tf(token, doc) * compute_idf(&token, self.docs.len(), &self.df);
            }
            if !rank.is_nan() {
                result.push((path.clone(), rank));
            }
            result.push((path.clone(), rank));
        }

        result.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
        result.reverse();
        result
    }

    pub fn add_document(
        &mut self,
        file_path: PathBuf,
        last_modified: SystemTime,
        content: &[char],
    ) {
        self.remove_document(&file_path);

        let mut tf = TermFreq::new();

        let mut count = 0;
        for term in Lexer::new(content) {
            if let Some(freq) = tf.get_mut(&term) {
                *freq += 1;
            } else {
                tf.insert(term, 1);
            }
            count += 1;
        }

        for t in tf.keys() {
            if let Some(freq) = self.df.get_mut(t) {
                *freq += 1;
            } else {
                self.df.insert(t.to_string(), 1);
            }
        }

        self.docs.insert(
            file_path,
            Doc {
                count,
                tf,
                last_modified,
            },
        );
    }
}

// refer wiki for following calculations reference
// https://en.wikipedia.org/wiki/Tf%E2%80%93idf
fn compute_tf(t: &str, doc: &Doc) -> f32 {
    let nume = doc.tf.get(t).cloned().unwrap_or(0) as f32;
    let deno = doc.count as f32;
    // WARN: following value is hardcoded for now.
    let const_k: f32 = 0.5; // this constant is to remove bias towards longer documents.
    const_k + (const_k * (nume / deno))
}

fn compute_idf(t: &str, n: usize, df: &DocFreq) -> f32 {
    let nume = n as f32;
    let deno = df.get(t).cloned().unwrap_or(1) as f32;
    (nume / deno).log10()
}
