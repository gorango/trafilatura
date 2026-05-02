use serde::{Deserialize, Serialize};
#[allow(unused)]
use napi_derive::napi;

#[derive(Debug, Serialize, Deserialize)]
#[napi(object)]
pub struct ExtractResultNapi {
    #[serde(rename = "contentText")]
    pub content_text: Option<String>,

    #[serde(rename = "contentHtml")]
    pub content_html: Option<String>,

    #[serde(rename = "contentMarkdown")]
    pub content_markdown: Option<String>,

    #[serde(rename = "commentsText")]
    pub comments_text: Option<String>,

    #[serde(rename = "commentsHtml")]
    pub comments_html: Option<String>,

    pub images: Vec<ImageDataNapi>,

    pub metadata: MetadataNapi,

    #[serde(rename = "classificationConfidence")]
    pub classification_confidence: Option<f64>,

    #[serde(rename = "extractionQuality")]
    pub extraction_quality: f64,

    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[napi(object)]
pub struct MetadataNapi {
    pub title: Option<String>,
    pub author: Option<String>,
    pub url: Option<String>,
    pub hostname: Option<String>,
    pub description: Option<String>,

    #[serde(rename = "sitename")]
    pub site_name: Option<String>,

    pub date: Option<String>,

    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub id: Option<String>,
    pub fingerprint: Option<String>,
    pub license: Option<String>,
    pub language: Option<String>,
    pub image: Option<String>,

    #[serde(rename = "pageType")]
    pub page_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[napi(object)]
pub struct ImageDataNapi {
    pub src: String,
    pub filename: String,
    pub alt: Option<String>,
    pub caption: Option<String>,

    #[serde(rename = "isHero")]
    pub is_hero: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[napi(object)]
pub struct OptionsNapi {
    #[serde(rename = "includeComments")]
    pub include_comments: Option<bool>,

    #[serde(rename = "includeTables")]
    pub include_tables: Option<bool>,

    #[serde(rename = "includeImages")]
    pub include_images: Option<bool>,

    #[serde(rename = "includeLinks")]
    pub include_links: Option<bool>,

    #[serde(rename = "favorPrecision")]
    pub favor_precision: Option<bool>,

    #[serde(rename = "favorRecall")]
    pub favor_recall: Option<bool>,

    #[serde(rename = "targetLanguage")]
    pub target_language: Option<String>,

    pub url: Option<String>,

    #[serde(rename = "authorBlacklist")]
    pub author_blacklist: Option<Vec<String>>,

    pub deduplicate: Option<bool>,

    #[serde(rename = "minExtractedSize")]
    pub min_extracted_size: Option<u32>,

    #[serde(rename = "minExtractedLen")]
    pub min_extracted_len: Option<u32>,

    #[serde(rename = "maxExtractedLen")]
    pub max_extracted_len: Option<u32>,

    #[serde(rename = "minOutputSize")]
    pub min_output_size: Option<u32>,

    #[serde(rename = "minOutputCommSize")]
    pub min_output_comm_size: Option<u32>,

    #[serde(rename = "minScore")]
    pub min_score: Option<u32>,

    #[serde(rename = "maxDuplicateRatio")]
    pub max_duplicate_ratio: Option<f64>,

    #[serde(rename = "maxLinkDensity")]
    pub max_link_density: Option<f64>,

    #[serde(rename = "minParagraphCluster")]
    pub min_paragraph_cluster: Option<u32>,

    #[serde(rename = "includeFormatting")]
    pub include_formatting: Option<bool>,

    #[serde(rename = "onlyWithMetadata")]
    pub only_with_metadata: Option<bool>,

    #[serde(rename = "maxTreeDepth")]
    pub max_tree_depth: Option<u32>,

    #[serde(rename = "minWordLength")]
    pub min_word_length: Option<u32>,

    #[serde(rename = "useFallbackExtraction")]
    pub use_fallback_extraction: Option<bool>,

    #[serde(rename = "dedupCacheSize")]
    pub dedup_cache_size: Option<u32>,

    #[serde(rename = "includeTitleInContent")]
    pub include_title_in_content: Option<bool>,

    #[serde(rename = "outputMarkdown")]
    pub output_markdown: Option<bool>,

    #[serde(rename = "pageType")]
    pub page_type: Option<String>,
}