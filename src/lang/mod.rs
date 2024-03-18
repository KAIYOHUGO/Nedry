use anyhow::Result;
use std::{collections::HashMap, io::Write, path::PathBuf, rc::Rc};

mod c;

pub trait LanguageExt {
    const EXT: &'static [&'static str];
}

pub trait Language {
    fn bundle(&self, input: PathBuf, output: &mut dyn Write) -> Result<()>;

    fn build(&self, input: PathBuf, output: Option<PathBuf>) -> Result<()>;

    fn run(&self, input: PathBuf) -> Result<()>;
}

pub fn lang_list() -> HashMap<String, Rc<Box<dyn Language>>> {
    let mut map = HashMap::new();
    add_lang::<c::C>(&mut map);
    map
}

fn add_lang<T: Default + Language + LanguageExt + 'static>(
    map: &mut HashMap<String, Rc<Box<dyn Language>>>,
) {
    let lang = Rc::new(Box::new(T::default()) as Box<dyn Language>);
    for ext in T::EXT {
        map.insert(ext.to_string(), lang.clone());
    }
}
