use std::borrow::Borrow;
use std::collections::HashMap;
#[allow(unused_imports)]
use std::env;
use std::hash::Hash;
use std::rc::Rc;

use sdl2::image::LoadTexture;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::ttf::{Font, Sdl2TtfContext};

pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;

    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct FontDetails {
    pub path: String,
    pub size: u16,
}

impl FontDetails {
    pub fn new(path: &str, size: u16) -> FontDetails {
        Self {
            path: path.to_string(),
            size,
        }
    }
}

impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> Self {
        Self {
            path: details.path.clone(),
            size: details.size,
        }
    }
}

pub type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;
pub type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

#[derive(Clone)]
pub struct ResourceManager<'l, K, R, L>
where
    K: Hash + Eq,
    L: 'l + ResourceLoader<'l, R>,
{
    loader: &'l L,
    cache: HashMap<K, Rc<R>>,
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L>
where
    K: Hash + Eq,
    L: ResourceLoader<'l, R>,
{
    pub fn new(loader: &'l L) -> Self {
        Self {
            cache: HashMap::new(),
            loader,
        }
    }

    pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
    where
        L: ResourceLoader<'l, R, Args = D>,
        D: Eq + Hash + ?Sized,
        K: Borrow<D> + for<'a> From<&'a D>,
    {
        self.cache.get(details).cloned().map_or_else(
            || {
                let resource = Rc::new(self.loader.load(details)?);
                self.cache.insert(details.into(), resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }
}

impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
    type Args = str;

    fn load(&'l self, path: &str) -> Result<Texture, String> {
        println!("Loading {}...", path);
        self.load_texture(path)
    }
}

impl<'l> ResourceLoader<'l, Font<'l, 'static>> for Sdl2TtfContext {
    type Args = FontDetails;

    fn load(&'l self, data: &FontDetails) -> Result<Font<'l, 'static>, String> {
        println!("Loading font {}...", data.path);
        self.load_font(&data.path, data.size)
    }
}
