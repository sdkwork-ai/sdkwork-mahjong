import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { isBlank } from '@sdkwork/utils';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const pcRoot = path.join(root, 'apps/sdkwork-mahjong-pc');
const packagesDir = path.join(pcRoot, 'packages');
const applicationCode = 'mahjong';

const allowedPrefixes = [
  `sdkwork-${applicationCode}-pc-core`,
  `sdkwork-${applicationCode}-pc-commons`,
  `sdkwork-${applicationCode}-pc-shell`,
  `sdkwork-${applicationCode}-pc-desktop`,
  `sdkwork-${applicationCode}-pc-console-core`,
  `sdkwork-${applicationCode}-pc-console-shell`,
  `sdkwork-${applicationCode}-pc-admin-core`,
  `sdkwork-${applicationCode}-pc-admin-shell`,
];

function isAllowedPcPackageName(name) {
  if (allowedPrefixes.includes(name)) {
    return true;
  }
  return (
    name.startsWith(`sdkwork-${applicationCode}-pc-console-`) ||
    name.startsWith(`sdkwork-${applicationCode}-pc-admin-`) ||
    (name.startsWith(`sdkwork-${applicationCode}-pc-`) &&
      !name.startsWith(`sdkwork-${applicationCode}-pc-console-`) &&
      !name.startsWith(`sdkwork-${applicationCode}-pc-admin-`))
  );
}

test('PC app root uses sdkwork-mahjong-pc directory and manifest', () => {
  assert.ok(fs.existsSync(path.join(pcRoot, 'agent.app.config.json')));
  const manifest = JSON.parse(
    fs.readFileSync(path.join(pcRoot, 'agent.app.config.json'), 'utf8'),
  );
  assert.equal(manifest.app.key, 'sdkwork-mahjong-pc');
  assert.equal(manifest.runtime.family, 'pc');
});

test('PC packages follow sdkwork-mahjong-pc-* naming', () => {
  const packageDirs = fs
    .readdirSync(packagesDir, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name);

  assert.ok(packageDirs.length > 0, 'expected PC packages under apps/sdkwork-mahjong-pc/packages');

  for (const dirName of packageDirs) {
    const packageJsonPath = path.join(packagesDir, dirName, 'package.json');
    assert.ok(fs.existsSync(packageJsonPath), `missing package.json for ${dirName}`);
    const pkg = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
    assert.equal(
      pkg.name,
      dirName,
      `package directory and package.json name must match: ${dirName}`,
    );
    assert.ok(
      isAllowedPcPackageName(pkg.name),
      `non-compliant PC package name: ${pkg.name}`,
    );
    assert.ok(
      !pkg.name.startsWith('sdkwork-game-'),
      `legacy sdkwork-game-* name remains: ${pkg.name}`,
    );
    assert.ok(!isBlank(pkg.name));
  }
});
