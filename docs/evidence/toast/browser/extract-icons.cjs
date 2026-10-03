// Run after installing this fixture's pinned package.json dependencies.
const path = require('path');
const fs = require('fs');
const os = require('os');
const {execFileSync} = require('child_process');
const temp = fs.mkdtempSync(path.join(os.tmpdir(), 'kumo-toast-icons-'));
try {
  const script = path.join(temp, 'extract.cjs');
  require('esbuild').buildSync({
    stdin: {contents: String.raw`
      import React from 'react';
      import {renderToStaticMarkup} from 'react-dom/server';
      import {CheckCircle, WarningOctagon, Warning, Info, X} from '@phosphor-icons/react';
      import fs from 'fs';
      import path from 'path';
      for (const [Icon, name] of [[CheckCircle,'success'],[WarningOctagon,'error'],[Warning,'warning'],[Info,'info'],[X,'close']]) {
        fs.writeFileSync(path.join(process.argv[2], 'toast-' + name + '.svg'),
          renderToStaticMarkup(React.createElement(Icon, {weight: Icon === X ? 'regular' : 'fill', color:'black'})) + '\n');
      }
    `, resolveDir: __dirname},
    bundle: true, platform: 'node', outfile: script,
    nodePaths: [path.join(__dirname, 'node_modules')]
  });
  execFileSync(process.execPath, [script, path.resolve(__dirname, '../../../../crates/gpui-kumo/assets')]);
} finally { fs.rmSync(temp, {recursive:true, force:true}); }
