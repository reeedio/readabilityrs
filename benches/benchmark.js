/**
 * Benchmark for Mozilla Readability.js
 *
 * This script benchmarks the performance of Mozilla's Readability.js library
 * on the same test cases used by readabilityrs for fair comparison.
 */

import { Readability } from '@mozilla/readability';
import { JSDOM } from 'jsdom';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const TEST_PAGES_DIR = path.join(__dirname, '..', 'tests', 'test-pages');

// Test cases grouped by size
// Note: wikipedia-2 is excluded due to excessive memory usage in JSDOM
const TEST_CASES = {
  small: ['001', '002', 'aclu'],
  medium: ['medium-1', 'nytimes-1', 'ars-1'],
  large: ['guardian-1', 'yahoo-2'],
  batch: ['001', '002', 'aclu', 'ars-1', 'bbc-1', 'buzzfeed-1', 'cnet', 'cnn', 'ehow-1', 'herald-sun-1']
};

function loadTestCase(name) {
  const filePath = path.join(TEST_PAGES_DIR, name, 'source.html');
  try {
    return fs.readFileSync(filePath, 'utf8');
  } catch (e) {
    return null;
  }
}

function parseWithReadability(html, url = 'https://example.com') {
  const dom = new JSDOM(html, { url });
  const reader = new Readability(dom.window.document);
  return reader.parse();
}

function benchmark(fn, iterations = 100) {
  // Warmup
  for (let i = 0; i < 3; i++) {
    fn();
  }

  // Force GC if available
  if (global.gc) {
    global.gc();
  }

  // Measure
  const times = [];
  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    fn();
    const end = performance.now();
    times.push(end - start);
  }

  times.sort((a, b) => a - b);

  return {
    mean: times.reduce((a, b) => a + b, 0) / times.length,
    median: times[Math.floor(times.length / 2)],
    min: times[0],
    max: times[times.length - 1],
    p95: times[Math.floor(times.length * 0.95)],
    iterations
  };
}

function formatMs(ms) {
  if (ms < 1) {
    return `${(ms * 1000).toFixed(2)} µs`;
  }
  return `${ms.toFixed(2)} ms`;
}

function formatBytes(bytes) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
}

async function runBenchmarks() {
  console.log('='.repeat(70));
  console.log('Mozilla Readability.js Benchmark');
  console.log('='.repeat(70));
  console.log();

  const results = {
    single: {},
    batch: null,
    large: {}
  };

  // Single document benchmarks by size
  console.log('Single Document Parsing');
  console.log('-'.repeat(70));

  for (const [size, cases] of Object.entries(TEST_CASES)) {
    if (size === 'batch') continue;

    for (const testCase of cases) {
      const html = loadTestCase(testCase);
      if (!html) {
        console.log(`Skipping ${testCase}: file not found`);
        continue;
      }

      const iterations = size === 'large' ? 10 : 50;
      const result = benchmark(() => parseWithReadability(html), iterations);

      results.single[testCase] = {
        ...result,
        size: html.length,
        sizeCategory: size
      };

      console.log(`${testCase.padEnd(20)} (${formatBytes(html.length).padStart(10)}) | ` +
        `mean: ${formatMs(result.mean).padStart(12)} | ` +
        `median: ${formatMs(result.median).padStart(12)} | ` +
        `p95: ${formatMs(result.p95).padStart(12)}`);
    }
  }

  console.log();

  // Skip batch processing to avoid memory issues
  // The single document results are sufficient for comparison
  console.log('Batch Processing: Skipped (memory-intensive with JSDOM)');
  console.log();

  // Large documents are already included in single results above
  // Copy them to large category for comparison script compatibility
  for (const testCase of TEST_CASES.large) {
    if (results.single[testCase]) {
      results.large[testCase] = results.single[testCase];
    }
  }

  console.log('='.repeat(70));

  // Output JSON results for comparison script
  const outputPath = path.join(__dirname, 'js-results.json');
  fs.writeFileSync(outputPath, JSON.stringify(results, null, 2));
  console.log(`Results saved to: ${outputPath}`);
}

runBenchmarks().catch(console.error);
