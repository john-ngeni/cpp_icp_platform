#!/usr/bin/env node
/**
 * Dhall Config Utility for CPP Projects
 *
 * Generates config/local.json from config/local.dhall (or other environments)
 * Reusable across all CPP repositories
 *
 * Usage:
 *   node tools/dhall-config.js generate [env]
 *   node tools/dhall-config.js deploy [env]
 *   node tools/dhall-config.js print [env]
 *   node tools/dhall-config.js args [env]
 *
 * Examples:
 *   node tools/dhall-config.js generate           # Generate config/local.json
 *   node tools/dhall-config.js generate preprod   # Generate config/preprod.json
 *   node tools/dhall-config.js deploy             # Deploy with local config
 *   node tools/dhall-config.js args preprod       # Print init args for preprod
 *
 * Authors: Fourth Transition Initiative
 * Operator: Cool Planet Foundation
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// Determine project root (where this script is called from)
const PROJECT_ROOT = process.cwd();
const CONFIG_DIR = path.join(PROJECT_ROOT, 'config');

function generateConfig(env = 'local') {
  const dhallFile = `${env}.dhall`;
  const jsonFile = `${env}.json`;

  console.log(`📝 Generating ${jsonFile} from ${dhallFile}...`);

  if (!fs.existsSync(path.join(CONFIG_DIR, dhallFile))) {
    console.error(`❌ ${CONFIG_DIR}/${dhallFile} not found`);
    process.exit(1);
  }

  try {
    // Run dhall-to-json from config directory (for relative imports to work)
    const json = execSync(
      `cd ${CONFIG_DIR} && ../node_modules/.bin/dhall-to-json < ${dhallFile}`,
      { encoding: 'utf-8' }
    );

    const outputPath = path.join(CONFIG_DIR, jsonFile);
    fs.writeFileSync(outputPath, json);
    console.log(`✅ Generated ${jsonFile}`);

    return JSON.parse(json);
  } catch (error) {
    console.error(`❌ Failed to generate config:`, error.message);
    process.exit(1);
  }
}

function loadConfig(env = 'local') {
  const jsonFile = `${env}.json`;
  const configPath = path.join(CONFIG_DIR, jsonFile);

  if (!fs.existsSync(configPath)) {
    console.log(`⚠️  ${jsonFile} not found, generating...`);
    return generateConfig(env);
  }

  return JSON.parse(fs.readFileSync(configPath, 'utf-8'));
}

function printConfig(env = 'local') {
  const config = loadConfig(env);
  console.log(JSON.stringify(config, null, 2));
}

function buildDeployArgs(env = 'local') {
  const config = loadConfig(env);
  const rc = config.runtime_config;

  if (!rc) {
    console.error('❌ No runtime_config found in config');
    process.exit(1);
  }

  // Build Candid record dynamically from config
  const fields = Object.entries(rc).map(([key, value]) => {
    if (value === null || value === undefined) {
      return `  ${key} = null`;
    } else if (typeof value === 'string') {
      return `  ${key} = "${value}"`;
    } else if (typeof value === 'number') {
      return `  ${key} = ${value}`;
    } else {
      return `  ${key} = ${JSON.stringify(value)}`;
    }
  }).join(';\n');

  return `(record {\n${fields};\n})`;
}

function deploy(env = 'local') {
  console.log(`🚀 Deploying to ${env} with runtime config...`);
  const config = generateConfig(env); // Regenerate to ensure fresh
  const args = buildDeployArgs(env);

  console.log('\nInit args:');
  console.log(args);
  console.log('');

  const network = env === 'local' ? '' : `--network ${env}`;

  try {
    execSync(`dfx deploy frontend ${network} --argument '${args}'`, {
      stdio: 'inherit',
      cwd: PROJECT_ROOT
    });
  } catch (error) {
    console.error('❌ Deployment failed');
    process.exit(1);
  }
}

// CLI
const command = process.argv[2] || 'generate';
const env = process.argv[3] || 'local';

switch (command) {
  case 'generate':
    generateConfig(env);
    break;
  case 'deploy':
    deploy(env);
    break;
  case 'print':
    printConfig(env);
    break;
  case 'args':
    console.log(buildDeployArgs(env));
    break;
  default:
    console.error(`Unknown command: ${command}`);
    console.error('Usage: dhall-config.js [generate|deploy|print|args] [env]');
    process.exit(1);
}
