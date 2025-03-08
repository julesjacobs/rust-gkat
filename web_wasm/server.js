const http = require('http');
const fs = require('fs');
const path = require('path');

const PORT = 8080;

const MIME_TYPES = {
  '.html': 'text/html',
  '.js': 'text/javascript',
  '.css': 'text/css',
  '.json': 'application/json',
  '.wasm': 'application/wasm',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.gif': 'image/gif',
  '.svg': 'image/svg+xml',
  '.ico': 'image/x-icon',
};

const server = http.createServer((req, res) => {
  // Log request with timestamp
  const timestamp = new Date().toISOString();
  console.log(`[${timestamp}] ${req.method} ${req.url}`);

  // Add CORS headers
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

  // Handle OPTIONS request for CORS preflight
  if (req.method === 'OPTIONS') {
    res.writeHead(204);
    res.end();
    return;
  }

  // Serve index.html for root path
  let filePath = req.url === '/' ? './index.html' : '.' + req.url;

  // Handle pkg directory requests
  if (req.url.startsWith('/pkg/')) {
    filePath = req.url.replace('/pkg/', './pkg/');
  }

  const extname = path.extname(filePath);
  const contentType = MIME_TYPES[extname] || 'text/plain';

  fs.readFile(filePath, (error, content) => {
    if (error) {
      if (error.code === 'ENOENT') {
        console.error(`[${timestamp}] File not found: ${filePath}`);

        // Try to serve index.html for SPA-like behavior
        if (req.url !== '/favicon.ico') {
          fs.readFile('./index.html', (err, indexContent) => {
            if (err) {
              res.writeHead(404);
              res.end('File not found');
            } else {
              res.writeHead(200, { 'Content-Type': 'text/html' });
              res.end(indexContent, 'utf-8');
            }
          });
        } else {
          res.writeHead(404);
          res.end('File not found');
        }
      } else {
        console.error(`[${timestamp}] Server error: ${error.code} for ${filePath}`);
        res.writeHead(500);
        res.end(`Server Error: ${error.code}`);
      }
    } else {
      // Add caching headers for better performance
      const headers = {
        'Content-Type': contentType,
        'Cache-Control': 'max-age=86400' // Cache for 1 day
      };

      // Don't cache HTML files
      if (extname === '.html') {
        headers['Cache-Control'] = 'no-cache';
      }

      res.writeHead(200, headers);
      res.end(content, 'utf-8');
      console.log(`[${timestamp}] Served: ${filePath} (${content.length} bytes)`);
    }
  });
});

// Handle server errors
server.on('error', (err) => {
  console.error(`Server error: ${err.message}`);
  if (err.code === 'EADDRINUSE') {
    console.error(`Port ${PORT} is already in use. Try a different port.`);
    process.exit(1);
  }
});

server.listen(PORT, () => {
  console.log(`
  ┌───────────────────────────────────────────┐
  │                                           │
  │   Rust-GKAT WebAssembly Demo Server       │
  │   Running at http://localhost:${PORT}/      │
  │                                           │
  │   Press Ctrl+C to stop                    │
  │                                           │
  └───────────────────────────────────────────┘
  `);
});