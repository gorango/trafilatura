use crate::types::*;
use rs_trafilatura::{ExtractResult, Metadata, Options, ImageData, page_type::PageType};

impl From<ExtractResult> for ExtractResultNapi {
    fn from(src: ExtractResult) -> Self {
        Self {
            content_text: Some(src.content_text).filter(|s| !s.is_empty()),
            content_html: src.content_html,
            content_markdown: src.content_markdown,
            comments_text: src.comments_text,
            comments_html: src.comments_html,
            images: src.images.into_iter().map(ImageDataNapi::from).collect(),
            metadata: MetadataNapi::from(src.metadata),
            classification_confidence: src.classification_confidence,
            extraction_quality: src.extraction_quality,
            warnings: src.warnings,
        }
    }
}

impl From<Metadata> for MetadataNapi {
    fn from(src: Metadata) -> Self {
        Self {
            title: src.title,
            author: src.author,
            url: src.url,
            hostname: src.hostname,
            description: src.description,
            site_name: src.sitename,
            date: src.date.map(|d| d.to_rfc3339()),
            categories: src.categories,
            tags: src.tags,
            id: src.id,
            fingerprint: src.fingerprint,
            license: src.license,
            language: src.language,
            image: src.image,
            page_type: src.page_type,
        }
    }
}

impl From<ImageData> for ImageDataNapi {
    fn from(src: ImageData) -> Self {
        Self {
            src: src.src,
            filename: src.filename,
            alt: src.alt,
            caption: src.caption,
            is_hero: src.is_hero,
        }
    }
}

impl From<OptionsNapi> for Options {
    fn from(src: OptionsNapi) -> Self {
        let mut opts = Options::default();

        if let Some(v) = src.include_comments { opts.include_comments = v; }
        if let Some(v) = src.include_tables { opts.include_tables = v; }
        if let Some(v) = src.include_images { opts.include_images = v; }
        if let Some(v) = src.include_links { opts.include_links = v; }
        if let Some(v) = src.favor_precision { opts.favor_precision = v; }
        if let Some(v) = src.favor_recall { opts.favor_recall = v; }
        if let Some(v) = src.target_language { opts.target_language = Some(v); }
        if let Some(v) = src.url { opts.url = Some(v); }
        if let Some(v) = src.author_blacklist { opts.author_blacklist = Some(v); }
        if let Some(v) = src.deduplicate { opts.deduplicate = v; }
        if let Some(v) = src.min_extracted_size { opts.min_extracted_size = v as usize; }
        if let Some(v) = src.min_extracted_len { opts.min_extracted_len = v as usize; }
        if let Some(v) = src.max_extracted_len { opts.max_extracted_len = v as usize; }
        if let Some(v) = src.min_output_size { opts.min_output_size = v as usize; }
        if let Some(v) = src.min_output_comm_size { opts.min_output_comm_size = v as usize; }
        if let Some(v) = src.min_score { opts.min_score = v as usize; }
        if let Some(v) = src.max_duplicate_ratio { opts.max_duplicate_ratio = v; }
        if let Some(v) = src.max_link_density { opts.max_link_density = v; }
        if let Some(v) = src.min_paragraph_cluster { opts.min_paragraph_cluster = v as usize; }
        if let Some(v) = src.include_formatting { opts.include_formatting = v; }
        if let Some(v) = src.only_with_metadata { opts.only_with_metadata = v; }
        if let Some(v) = src.max_tree_depth { opts.max_tree_depth = v as usize; }
        if let Some(v) = src.min_word_length { opts.min_word_length = v as usize; }
        if let Some(v) = src.use_fallback_extraction { opts.use_fallback_extraction = v; }
        if let Some(v) = src.dedup_cache_size { opts.dedup_cache_size = v as usize; }
        if let Some(v) = src.include_title_in_content { opts.include_title_in_content = v; }
        if let Some(v) = src.output_markdown { opts.output_markdown = v; }
        if let Some(v) = src.page_type {
            opts.page_type = match v.as_str() {
                "article" => Some(PageType::Article),
                "forum" => Some(PageType::Forum),
                "product" => Some(PageType::Product),
                "collection" => Some(PageType::Category),
                "listing" => Some(PageType::Listing),
                "documentation" => Some(PageType::Documentation),
                "service" => Some(PageType::Service),
                _ => None,
            };
        }

        opts
    }
}