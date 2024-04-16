use super::lexer::Lexer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::result::Result;
use std::time::SystemTime;

pub trait Model {
    fn search_query(&self, query: &[char]) -> Result<Vec<(PathBuf, f32)>, ()>;
    fn requires_reindexing(&mut self, path: &Path, last_modified: SystemTime) -> Result<bool, ()>;
    fn add_document(
        &mut self,
        path: PathBuf,
        last_modified: SystemTime,
        content: &[char],
    ) -> Result<(), ()>;
}

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
pub struct InMemoryModel {
    docs: Docs,
    df: DocFreq,
}

impl InMemoryModel {
    fn remove_document(&mut self, _file_path: &Path) {
        todo!();
    }
}

impl Model for InMemoryModel {
    fn requires_reindexing(
        &mut self,
        file_path: &Path,
        last_modified: SystemTime,
    ) -> Result<bool, ()> {
        if let Some(doc) = self.docs.get(file_path) {
            return Ok(doc.last_modified < last_modified);
        }
        return Ok(true);
    }

    fn search_query(&self, query: &[char]) -> Result<Vec<(PathBuf, f32)>, ()> {
        let mut result = Vec::new();
        let tokens = Lexer::new(&query).collect::<Vec<_>>();
        for (path, doc) in &self.docs {
            let mut rank = 0f32;
            for token in &tokens {
                rank += compute_tf(token, doc) * compute_idf(&token, self.docs.len(), &self.df);
            }
            result.push((path.clone(), rank));
        }
        result.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
        result.reverse();
        Ok(result)
    }

    fn add_document(
        &mut self,
        file_path: PathBuf,
        last_modified: SystemTime,
        content: &[char],
    ) -> Result<(), ()> {
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
        Ok(())
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
