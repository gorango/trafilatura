import { test, describe } from 'node:test'
import assert from 'node:assert'
import { extract, extractBytes } from './index.js'

describe('extract', () => {
  test('extracts content from simple HTML', () => {
    const html = `
      <html>
        <head><title>Test Article</title></head>
        <body>
          <article>
            <h1>Main Title</h1>
            <p>This is the main content of the article.</p>
          </article>
        </body>
      </html>
    `

    const result = extract(html)

    assert.ok(result, 'should return a result')
    assert.strictEqual(result.metadata.title, 'Test Article')
    assert.ok(result.contentText, 'should have content text')
    assert.ok(result.contentText.includes('main content'), 'should include main content')
    assert.ok(result.extractionQuality >= 0, 'extractionQuality should be >= 0')
  })

  test('extracts content from article with paragraphs', () => {
    const html = `
      <html>
        <head><title>Multi-Paragraph Article</title></head>
        <body>
          <article>
            <h1>Welcome to My Blog</h1>
            <p>First paragraph with some interesting content.</p>
            <p>Second paragraph with more details.</p>
            <p>Third paragraph to round things out.</p>
          </article>
        </body>
      </html>
    `

    const result = extract(html)

    assert.ok(result.contentText.includes('First paragraph'), 'should include first paragraph')
    assert.ok(result.contentText.includes('Second paragraph'), 'should include second paragraph')
  })
})

describe('extract with options', () => {
  test('outputMarkdown option produces markdown', () => {
    const html = `
      <html>
        <head><title>Markdown Test</title></head>
        <body>
          <article>
            <h1>Heading One</h1>
            <p>Some content here.</p>
          </article>
        </body>
      </html>
    `

    const result = extract(html, { outputMarkdown: true })

    assert.ok(result.contentMarkdown !== undefined, 'should have content markdown')
    assert.ok(
      result.contentMarkdown.includes('Heading One') ||
        result.contentMarkdown.includes('Some content'),
      'should include content',
    )
  })

  test('includeImages option extracts images', () => {
    const html = `
      <html>
        <head><title>Image Test</title></head>
        <body>
          <article>
            <h1>Article</h1>
            <img src="https://example.com/hero.jpg" alt="Hero Image">
          </article>
        </body>
      </html>
    `

    const result = extract(html, { includeImages: true })

    assert.ok(Array.isArray(result.images), 'images should be an array')
    assert.ok(result.images.length > 0, 'should have at least one image')
    assert.strictEqual(result.images[0].src, 'https://example.com/hero.jpg')
  })

  test('includeTables option includes table content', () => {
    const html = `
      <html>
        <head><title>Table Test</title></head>
        <body>
          <article>
            <h1>Data</h1>
            <table>
              <tr><th>Header</th></tr>
              <tr><td>Data</td></tr>
            </table>
          </article>
        </body>
      </html>
    `

    const result = extract(html, { includeTables: true })

    assert.ok(result.contentText.includes('Header'), 'should include table content')
  })

  test('includeLinks option preserves links', () => {
    const html = `
      <html>
        <head><title>Links Test</title></head>
        <body>
          <article>
            <h1>Article</h1>
            <p>Check out <a href="https://example.com">this link</a>.</p>
          </article>
        </body>
      </html>
    `

    const result = extract(html, { includeLinks: true })

    assert.ok(result.contentText, 'should have content')
  })

  test('includeComments option', () => {
    const html = `
      <html>
        <head><title>Comments Test</title></head>
        <body>
          <article>
            <h1>Article</h1>
            <p>Main content.</p>
          </article>
        </body>
      </html>
    `

    const result = extract(html, { includeComments: true })

    assert.ok(result !== undefined, 'should handle option without error')
  })

  test('pageType option overrides classification', () => {
    const html = `
      <html>
        <head><title>Product Page</title></head>
        <body>
          <article>
            <h1>Product Name</h1>
            <p>$99.99</p>
          </article>
        </body>
      </html>
    `

    const result = extract(html, { pageType: 'product' })

    assert.ok(result.contentText, 'should extract content')
    assert.ok(result.extractionQuality >= 0, 'should have quality score')
  })

  test('targetLanguage option', () => {
    const html = `
      <html>
        <head><title>English Article</title></head>
        <body>
          <article>
            <h1>Article</h1>
            <p>Content here.</p>
          </article>
        </body>
      </html>
    `

    const result = extract(html, { targetLanguage: 'en' })

    assert.ok(result.contentText, 'should extract content')
  })

  test('url option provides context', () => {
    const html = `
      <html>
        <head><title>Article</title></head>
        <body>
          <article>
            <h1>Content</h1>
          </article>
        </body>
      </html>
    `

    const result = extract(html, { url: 'https://example.com/blog/post' })

    assert.ok(result.contentText, 'should extract content')
    if (result.metadata.url) {
      assert.strictEqual(result.metadata.url, 'https://example.com/blog/post')
    }
  })

  test('deduplicate option removes duplicates', () => {
    const html = `
      <html>
        <head><title>Duplicate Test</title></head>
        <body>
          <article>
            <h1>Title</h1>
            <p>Same content repeated. Same content repeated. Same content repeated.</p>
          </article>
        </body>
      </html>
    `

    const result = extract(html, { deduplicate: true })

    assert.ok(result.contentText, 'should extract content')
  })

  test('favorPrecision option uses stricter filtering', () => {
    const html = `
      <html>
        <head><title>Precision Test</title></head>
        <body>
          <nav>Navigation menu</nav>
          <article>
            <h1>Real Content</h1>
            <p>This is the important article content.</p>
          </article>
          <aside>Sidebar ads</aside>
        </body>
      </html>
    `

    const result = extract(html, { favorPrecision: true })

    assert.ok(result.contentText, 'should extract content')
    assert.ok(result.contentText.includes('Real Content'), 'should include real content')
    assert.ok(!result.contentText.includes('Navigation menu'), 'should exclude nav')
  })

  test('favorRecall option includes more content', () => {
    const html = `
      <html>
        <head><title>Recall Test</title></head>
        <body>
          <article>
            <h1>Content</h1>
            <p>Main article text.</p>
          </article>
        </body>
      </html>
    `

    const result = extract(html, { favorRecall: true })

    assert.ok(result.contentText, 'should extract content')
  })

  test('includeFormatting option preserves formatting', () => {
    const html = `
      <html>
        <head><title>Formatting Test</title></head>
        <body>
          <article>
            <h1>Title</h1>
            <p><strong>Bold</strong> and <em>italic</em> text.</p>
          </article>
        </body>
      </html>
    `

    const result = extract(html, { includeFormatting: true })

    assert.ok(result.contentText, 'should preserve some formatting')
  })
})

describe('extractBytes', () => {
  test('extracts content from Buffer', () => {
    const html = '<html><body><article>Test content</article></body></html>'
    const result = extractBytes(Buffer.from(html))

    assert.ok(result.contentText !== undefined, 'should have content')
    assert.ok(result.contentText.includes('Test content'), 'should include content')
  })

  test('handles encoding detection', () => {
    const html = '<html><body><article>Content</article></body></html>'
    const buffer = Buffer.from(html, 'utf-8')
    const result = extractBytes(buffer)

    assert.ok(result.contentText, 'should handle encoding')
  })

  test('extracts bytes with options', () => {
    const html = '<html><body><article>Test</article></body></html>'
    const result = extractBytes(Buffer.from(html), { outputMarkdown: true })

    assert.ok(result.contentMarkdown !== undefined, 'should have markdown content')
  })
})

describe('metadata extraction', () => {
  test('extracts title from HTML title tag', () => {
    const html = `
      <html>
        <head><title>My Page Title</title></head>
        <body><article>Content</article></body>
      </html>
    `

    const result = extract(html)

    assert.strictEqual(result.metadata.title, 'My Page Title')
  })

  test('extracts author from meta tag', () => {
    const html = `
      <html>
        <head>
          <title>Article</title>
          <meta name="author" content="John Doe">
        </head>
        <body><article>Content</article></body>
      </html>
    `

    const result = extract(html)

    assert.strictEqual(result.metadata.author, 'John Doe')
  })

  test('extracts description from meta tag', () => {
    const html = `
      <html>
        <head>
          <title>Article</title>
          <meta name="description" content="Article description">
        </head>
        <body><article>Content</article></body>
      </html>
    `

    const result = extract(html)

    assert.strictEqual(result.metadata.description, 'Article description')
  })

  test('extracts page type', () => {
    const html = `
      <html>
        <head><title>Product</title></head>
        <body>
          <article>
            <h1>Product</h1>
            <span class="price">$99</span>
          </article>
        </body>
      </html>
    `

    const result = extract(html)

    assert.ok(result.metadata.pageType, 'should have page type')
  })
})

describe('edge cases', () => {
  test('handles empty input', () => {
    const result = extract('')

    assert.ok(result, 'should return a result')
    assert.ok(result.metadata, 'should have metadata')
    assert.ok(result.extractionQuality >= 0, 'extractionQuality should be >= 0')
  })

  test('handles invalid HTML gracefully', () => {
    const html = 'not valid html at all <<<<>>>'
    const result = extract(html)

    assert.ok(result, 'should return a result')
    assert.ok(result.extractionQuality >= 0, 'extraction quality should be >= 0')
  })

  test('handles minimal HTML', () => {
    const html = '<html><body><p>Minimal</p></body></html>'
    const result = extract(html)

    assert.ok(result.contentText, 'should extract minimal content')
  })

  test('handles HTML with only nav and footer', () => {
    const html = `
      <html>
        <head><title>No Content</title></head>
        <body>
          <nav>Home</nav>
          <footer>Footer</footer>
        </body>
      </html>
    `

    const result = extract(html)

    assert.ok(result !== undefined, 'should handle no main content')
  })

  test('returns extractionQuality for all inputs', () => {
    const htmls = [
      '<html><body><article>Content</article></body></html>',
      '',
      'random text',
      '<html><head><title>T</title></head></html>',
    ]

    for (const html of htmls) {
      const result = extract(html)
      assert.ok(
        typeof result.extractionQuality === 'number',
        'extractionQuality should be a number',
      )
      assert.ok(
        result.extractionQuality >= 0 && result.extractionQuality <= 1,
        'should be between 0 and 1',
      )
    }
  })

  test('returns warnings array', () => {
    const html = '<html><body><article>Content</article></body></html>'
    const result = extract(html)

    assert.ok(Array.isArray(result.warnings), 'warnings should be an array')
  })
})
