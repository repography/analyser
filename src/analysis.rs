use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct User {
	pub name: String,
	pub contributions: usize,
}

impl User {
	pub fn new(name: String) -> User {
		User {
			name,
			contributions: 0,
		}
	}
}

#[derive(Debug, Clone, Serialize)]
pub struct Change {
	#[serde(rename = "File")]
	pub file: usize,
	#[serde(rename = "Additions")]
	pub additions: usize,
	#[serde(rename = "Deletions")]
	pub deletions: usize,
}

pub fn serialize_date<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::ser::Serializer,
{
	let s = date.to_rfc3339();
	serializer.serialize_str(&s)
}

#[derive(Debug, Serialize)]
pub struct Commit {
	pub id: String,
	#[serde(serialize_with = "serialize_date")]
	pub time: DateTime<FixedOffset>,
	pub user: usize,
	pub files: Vec<Change>,
	pub words: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Language {
	pub name: String,
	pub bytes: usize,
}

#[derive(Debug, Serialize)]
pub struct Analysis {
	pub commits: Vec<Commit>,
	#[serde(skip)]
	pub files_map: HashMap<String, usize>,
	pub files: Vec<String>,
	#[serde(skip)]
	pub users_map: HashMap<String, usize>,
	pub users: Vec<User>,
	pub languages: Vec<Language>,
	pub revision: String,
	#[serde(rename = "totalBytes")]
	pub total_bytes: usize,
	#[serde(skip)]
	stop_words: HashMap<&'static str, u8>,
}

impl Analysis {
	pub fn new(revision: &str) -> Analysis {
		Analysis {
			commits: vec![],
			files_map: HashMap::new(),
			files: vec![],
			users_map: HashMap::new(),
			users: vec![],
			languages: vec![],
			revision: revision.to_owned(),
			total_bytes: 0,
			stop_words: get_stop_words(),
		}
	}

	pub fn get_file_index(&mut self, path: &str) -> usize {
		if self.files_map.contains_key(path) {
			self.files_map[path]
		} else {
			let index = self.files.len();
			self.files.push(path.to_string());
			self.files_map.insert(path.to_string(), index);
			index
		}
	}

	pub fn get_user_index(&mut self, name: &str) -> usize {
		if self.users_map.contains_key(name) {
			self.users_map[name]
		} else {
			let index = self.users.len();
			self.users.push(User::new(name.to_string()));
			self.users_map.insert(name.to_string(), index);
			index
		}
	}

	pub fn filter_stop_words<'a, I>(&self, words: I) -> Vec<String>
	where
		I: Iterator<Item = &'a str>,
	{
		let mut filtered = vec![];
		for word in words {
			if word.len() < 2 {
				continue;
			}

			if !word.chars().any(|c| c.is_alphabetic()) {
				continue;
			}

			let lower = word.to_lowercase();
			if !self.stop_words.contains_key::<str>(&lower) {
				filtered.push(lower);
			}
		}
		filtered
	}
}

fn get_stop_words() -> HashMap<&'static str, u8> {
	HashMap::from([
		("i", 0),
		("me", 0),
		("my", 0),
		("myself", 0),
		("we", 0),
		("us", 0),
		("our", 0),
		("ours", 0),
		("ourselves", 0),
		("you", 0),
		("your", 0),
		("yours", 0),
		("yourself", 0),
		("yourselves", 0),
		("he", 0),
		("him", 0),
		("his", 0),
		("himself", 0),
		("she", 0),
		("her", 0),
		("hers", 0),
		("herself", 0),
		("it", 0),
		("its", 0),
		("itself", 0),
		("they", 0),
		("them", 0),
		("their", 0),
		("theirs", 0),
		("themselves", 0),
		("what", 0),
		("which", 0),
		("who", 0),
		("whom", 0),
		("whose", 0),
		("this", 0),
		("that", 0),
		("these", 0),
		("those", 0),
		("am", 0),
		("is", 0),
		("are", 0),
		("was", 0),
		("were", 0),
		("be", 0),
		("been", 0),
		("being", 0),
		("have", 0),
		("has", 0),
		("had", 0),
		("having", 0),
		("do", 0),
		("does", 0),
		("did", 0),
		("doing", 0),
		("will", 0),
		("would", 0),
		("should", 0),
		("can", 0),
		("could", 0),
		("ought", 0),
		("i'm", 0),
		("you're", 0),
		("he's", 0),
		("she's", 0),
		("it's", 0),
		("we're", 0),
		("they're", 0),
		("i've", 0),
		("you've", 0),
		("we've", 0),
		("they've", 0),
		("i'd", 0),
		("you'd", 0),
		("he'd", 0),
		("she'd", 0),
		("we'd", 0),
		("they'd", 0),
		("i'll", 0),
		("you'll", 0),
		("he'll", 0),
		("she'll", 0),
		("we'll", 0),
		("they'll", 0),
		("isn't", 0),
		("aren't", 0),
		("wasn't", 0),
		("weren't", 0),
		("hasn't", 0),
		("haven't", 0),
		("hadn't", 0),
		("doesn't", 0),
		("don't", 0),
		("didn't", 0),
		("won't", 0),
		("wouldn't", 0),
		("shan't", 0),
		("shouldn't", 0),
		("can't", 0),
		("cannot", 0),
		("couldn't", 0),
		("mustn't", 0),
		("let's", 0),
		("that's", 0),
		("who's", 0),
		("what's", 0),
		("here's", 0),
		("there's", 0),
		("when's", 0),
		("where's", 0),
		("why's", 0),
		("how's", 0),
		("a", 0),
		("an", 0),
		("the", 0),
		("and", 0),
		("but", 0),
		("if", 0),
		("or", 0),
		("because", 0),
		("as", 0),
		("until", 0),
		("while", 0),
		("of", 0),
		("at", 0),
		("by", 0),
		("for", 0),
		("with", 0),
		("about", 0),
		("against", 0),
		("between", 0),
		("into", 0),
		("through", 0),
		("during", 0),
		("before", 0),
		("after", 0),
		("above", 0),
		("below", 0),
		("to", 0),
		("from", 0),
		("up", 0),
		("upon", 0),
		("down", 0),
		("in", 0),
		("out", 0),
		("on", 0),
		("off", 0),
		("over", 0),
		("under", 0),
		("again", 0),
		("further", 0),
		("then", 0),
		("once", 0),
		("here", 0),
		("there", 0),
		("when", 0),
		("where", 0),
		("why", 0),
		("how", 0),
		("all", 0),
		("any", 0),
		("both", 0),
		("each", 0),
		("few", 0),
		("more", 0),
		("most", 0),
		("other", 0),
		("some", 0),
		("such", 0),
		("no", 0),
		("nor", 0),
		("not", 0),
		("only", 0),
		("own", 0),
		("same", 0),
		("so", 0),
		("than", 0),
		("too", 0),
		("very", 0),
		("say", 0),
		("says", 0),
		("said", 0),
		("shall", 0),
		("add", 0),
		("fix", 0),
		("remove", 0),
		("change", 0),
		("improve", 0),
		("use", 0),
		("test", 0),
		("tests", 0),
		("update", 0),
		("move", 0),
		("support", 0),
		("implement", 0),
		("readme", 0),
		("fixes", 0),
		("fixed", 0),
		("make", 0),
		("warning", 0),
		("error", 0),
		("pull", 0),
		("merge", 0),
		("merged", 0),
		("request", 0),
		("generate", 0),
		("group", 0),
		("panic", 0),
		("doc", 0),
		("chore", 0),
		("add", 0),
		("issue", 0),
		("increase", 0),
		("version", 0),
		("new", 0),
		("master", 0),
		("branch", 0),
		("bug", 0),
		("release", 0),
		("bump", 0),
		("commit", 0),
		("refactor", 0),
		("prepare", 0),
		("github.com", 0),
		("added", 0),
		("git", 0),
		("new", 0),
		("patch", 0),
		("patches", 0),
		("patched", 0),
		("need", 0),
		("needed", 0),
		("roll", 0),
		("back", 0),
		("instead", 0),
		("is", 0),
		("yours", 0),
		("its", 0),
		("whos", 0),
		("ups", 0),
		("downs", 0),
		("ins", 0),
		("outs", 0),
		("ons", 0),
		("offs", 0),
		("overs", 0),
		("unders", 0),
		("wheres", 0),
		("others", 0),
		("owns", 0),
		("adds", 0),
		("fixes", 0),
		("removes", 0),
		("changes", 0),
		("improves", 0),
		("uses", 0),
		("tests", 0),
		("updates", 0),
		("moves", 0),
		("supports", 0),
		("implements", 0),
		("readmes", 0),
		("makes", 0),
		("warnings", 0),
		("errors", 0),
		("pulls", 0),
		("merges", 0),
		("requests", 0),
		("generates", 0),
		("groups", 0),
		("panics", 0),
		("docs", 0),
		("chores", 0),
		("issues", 0),
		("increases", 0),
		("versions", 0),
		("masters", 0),
		("bugs", 0),
		("releases", 0),
		("bumps", 0),
		("commits", 0),
		("refactors", 0),
		("prepares", 0),
		("news", 0),
		("patches", 0),
		("needs", 0),
		("rolls", 0),
	])
}
