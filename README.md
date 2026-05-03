# napi-rs-trafilatura

Fast and accurate web content extraction for Node.js.

High-performance NAPI bindings for [rs-trafilatura](https://github.com/Murrough-Foley/rs-trafilatura) - a Rust port of [trafilatura](https://github.com/adbar/trafilatura). Extracts clean, readable content from web pages while removing boilerplate, navigation, and advertisements.

## Features

- **Fast**: 71 files/s for articles, 46 files/s overall (native Rust)
- **Accurate**: F1 0.966 on ScrapingHub benchmark, F1 0.859 across 7 page types
- **Page Type Classification**: Auto-detects 7 page types (article, forum, product, collection, listing, documentation, service)
- **Per-Type Extraction**: Specialized extraction profiles for each page type
- **Extraction Quality Predictor**: ML-based confidence scoring (0.0-1.0)
- **Markdown Output**: GitHub Flavored Markdown with headings, lists, tables, bold/italic, code blocks
- **Rich Metadata**: Title, author, date, description, categories, tags, license, images from JSON-LD, Open Graph, Dublin Core
- **Configurable**: 28 options to tune precision/recall tradeoff, content selection, and output format
- **Robust**: Handles malformed HTML with automatic character encoding detection

## Installation

```bash
npm install trafilatura
```

## Usage

```javascript
import { extract } from 'trafilatura'

const html = `
<html>
  <head><title>Example Article</title></head>
  <body>
    <nav>Home | About | Contact</nav>
    <article>
      <h1>Main Title</h1>
      <p>This is the main content of the article.</p>
    </article>
    <footer>Copyright 2024</footer>
  </body>
</html>
`

const result = extract(html)
console.log('Title:', result.metadata.title)
console.log('Content:', result.contentText)
console.log('Page type:', result.metadata.pageType)
console.log('Quality:', result.extractionQuality)
```

### With Options

```javascript
import { extractWithOptions } from 'trafilatura'

const result = extractWithOptions(html, {
  outputMarkdown: true,
  includeImages: true,
  favorPrecision: true,
  url: 'https://example.com/article',
})

console.log(result.contentMarkdown)
console.log(result.images)
```

### Page Type Override

```javascript
import { extractWithOptions } from 'trafilatura'

const result = extractWithOptions(html, {
  pageType: 'product', // Force product page extraction
})
```

### Working with Bytes

For HTML with unknown encoding:

```javascript
import { extractBytes } from 'trafilatura'

const htmlBuffer = await fs.promises.readFile('page.html')
const result = extractBytes(htmlBuffer)
```

## API

### extract(html: string): ExtractResult

Extract content from HTML string.

### extractWithOptions(html: string, options?: Options): ExtractResult

Extract content with options.

### extractBytes(buffer: Buffer): ExtractResult

Extract content from Buffer (handles encoding detection).

### extractBytesWithOptions(buffer: Buffer, options?: Options): ExtractResult

Extract content from Buffer with options.

## ExtractResult

| Field                    | Type        | Description                             |
| ------------------------ | ----------- | --------------------------------------- |
| contentText              | string?     | Main content as plain text              |
| contentHtml              | string?     | Main content as HTML                    |
| contentMarkdown          | string?     | Main content as Markdown                |
| commentsText             | string?     | Comments section as text                |
| commentsHtml             | string?     | Comments section as HTML                |
| images                   | ImageData[] | Extracted images                        |
| metadata                 | Metadata    | Extracted metadata                      |
| classificationConfidence | number?     | ML classifier confidence (0.0-1.0)      |
| extractionQuality        | number      | Extraction quality confidence (0.0-1.0) |
| warnings                 | string[]    | Processing warnings                     |

## Options

| Option                | Type     | Description                      |
| --------------------- | -------- | -------------------------------- |
| includeComments       | boolean  | Include comments in output       |
| includeTables         | boolean  | Include tables                   |
| includeImages         | boolean  | Include images                   |
| includeLinks          | boolean  | Include links                    |
| favorPrecision        | boolean  | Favor precision over recall      |
| favorRecall           | boolean  | Favor recall over precision      |
| targetLanguage        | string   | Target language code             |
| url                   | string   | Source URL                       |
| authorBlacklist       | string[] | Author names to exclude          |
| deduplicate           | boolean  | Remove duplicate content         |
| minExtractedSize      | number   | Minimum extracted content size   |
| minExtractedLen       | number   | Minimum extracted length         |
| maxExtractedLen       | number   | Maximum extracted length         |
| minOutputSize         | number   | Minimum output size              |
| minOutputCommSize     | number   | Minimum comments size            |
| minScore              | number   | Minimum quality score            |
| maxDuplicateRatio     | number   | Max duplicate ratio threshold    |
| maxLinkDensity        | number   | Max link density threshold       |
| minParagraphCluster   | number   | Min paragraph cluster size       |
| includeFormatting     | boolean  | Include text formatting          |
| onlyWithMetadata      | boolean  | Only extract pages with metadata |
| maxTreeDepth          | number   | Maximum DOM tree depth           |
| minWordLength         | number   | Minimum word length              |
| useFallbackExtraction | boolean  | Use fallback extraction          |
| dedupCacheSize        | number   | Deduplication cache size         |
| includeTitleInContent | boolean  | Include title in content         |
| outputMarkdown        | boolean  | Output as Markdown               |
| pageType              | string   | Override page type               |

## Build from Source

```bash
# Install build dependencies
npm install

# Build the native module
npm run build
```

## Test

```bash
npm test
```

## License

MIT

## Acknowledgments

- [trafilatura](https://github.com/adbar/trafilatura) - Original Python implementation by Adrien Barbaresi
- [rs-trafilatura](https://github.com/Murrough-Foley/rs-trafilatura) - Rust port by Murrough Foley
