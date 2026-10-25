import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { isBlank, slugify } from '@sdkwork/utils';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

test('workspace exposes agent.app.config.json with game domain', () => {
  const manifest = JSON.parse(
    fs.readFileSync(path.join(root, 'agent.app.config.json'), 'utf8'),
  );
  assert.equal(manifest.app.domain, 'game');
  assert.ok(!isBlank(manifest.sdk.appSdkFamily));
});

test('route manifests declare WebRequestContext on protected routes', () => {
  const manifestPath = path.join(
    root,
    'sdks/_route-manifests/app-api/sdkwork-routes-mahjong-app-api.route-manifest.json',
  );
  const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
  const protectedRoute = manifest.routes.find((route) => route.path.includes('/mahjong/matches'));
  assert.equal(protectedRoute.requestContext, 'WebRequestContext');
});

test('utils slugify normalizes game codes', () => {
  assert.equal(slugify('Demo Game 2026'), 'demo-game-2026');
});
