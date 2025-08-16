use chrono::prelude::*;
use chrono::IsoWeek;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use serde_json;

#[derive(Debug, Deserialize)]
struct Author {
    login: String,
}

#[derive(Debug, Deserialize)]
struct CommitAuthor {
    date: DateTime<FixedOffset>,
}

#[derive(Debug, Deserialize)]
struct Commit {
    author: CommitAuthor,
}

#[derive(Debug, Deserialize)]
pub struct CommitData {
    author: Author,
    commit: Commit,
}

pub fn commits_per_author(data: &[CommitData]) -> HashMap<String, u32> {
    let mut commits_per_author = HashMap::new();
    for commit_data in data {
        *commits_per_author.entry(commit_data.author.login.clone()).or_insert(0) += 1;
    }
    commits_per_author
}

pub fn commits_per_week(data: &[CommitData]) -> HashMap<String, u32> {
    let mut commits_per_week = HashMap::new();
    for commit_data in data {
        let week = commit_data.commit.author.date.iso_week();
        let week_string = format!("{}-W{}", week.year(), week.week());
        *commits_per_week.entry(week_string).or_insert(0) += 1;
    }
    commits_per_week
}