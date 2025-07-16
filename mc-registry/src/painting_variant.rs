#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct PaintingVariant {
    pub asset_id: String,
    pub title: TextComponent,
    pub author: Option<TextComponent>,
    pub width: i32,
    pub height: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct TextComponent {
    pub translate: String,
    pub color: Option<String>,
}

impl simdnbt::Serialize for PaintingVariant {
    fn to_compound(self) -> simdnbt::owned::NbtCompound {
        let mut comp = simdnbt::owned::NbtCompound::new();

        comp.insert("asset_id", self.asset_id);

        let mut title_comp = simdnbt::owned::NbtCompound::new();
        title_comp.insert("translate", self.title.translate);
        if let Some(color) = self.title.color {
            title_comp.insert("color", color);
        }
        comp.insert("title", title_comp);

        if let Some(author) = self.author {
            let mut author_comp = simdnbt::owned::NbtCompound::new();
            author_comp.insert("translate", author.translate);
            if let Some(color) = author.color {
                author_comp.insert("color", color);
            }
            comp.insert("author", author_comp);
        }

        comp.insert("width", self.width);
        comp.insert("height", self.height);

        comp
    }
}
