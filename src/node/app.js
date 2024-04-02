import { say } from '../pkg/package_buddy.js';

import http from 'http';
import url from 'url';
const hostname = '127.0.0.1';
const port = 8080;

const server = http.createServer((req, res) => {
  const queryObject = url.parse(req.url,true).query;
  res.statusCode = 200;
  console.log(queryObject['name'])
  res.setHeader('Content-Type', 'text/plain');
  try {
    res.end(say(queryObject['name']));
  } catch (error) {
    res.end(toString(error));
  }
  
});
//initSync('../pkg/package_buddy_bg.wasm');
server.listen(port, hostname, () => {});