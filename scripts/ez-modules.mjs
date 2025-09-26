#!/usr/bin/env node

/**
 * EZ Tauri Module Management CLI
 *
 * This CLI tool helps developers manage modules in the EZ Tauri application.
 * It provides commands to list, install, enable, disable, and configure modules.
 */

import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { readFileSync, writeFileSync, existsSync, mkdirSync, readdirSync, cpSync, rmSync, statSync } from 'fs';
import { execSync } from 'child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const projectRoot = dirname(__dirname);

// ANSI color codes for better CLI output
const colors = {
    reset: '\x1b[0m',
    red: '\x1b[31m',
    green: '\x1b[32m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    magenta: '\x1b[35m',
    cyan: '\x1b[36m',
    white: '\x1b[37m',
    gray: '\x1b[90m',
    bold: '\x1b[1m',
};

function colorize(text, color) {
    return `${colors[color]}${text}${colors.reset}`;
}

function logInfo(message) {
    console.log(`${colorize('ℹ', 'cyan')} ${message}`);
}

function logSuccess(message) {
    console.log(`${colorize('✓', 'green')} ${message}`);
}

function logWarning(message) {
    console.log(`${colorize('⚠', 'yellow')} ${message}`);
}

function logError(message) {
    console.log(`${colorize('✗', 'red')} ${message}`);
}

function getModulesDirectory() {
    return join(projectRoot, 'modules');
}

function getConfigPath() {
    return join(projectRoot, 'module_config.json');
}

function loadModuleConfig() {
    const configPath = getConfigPath();
    if (!existsSync(configPath)) {
        return {};
    }

    try {
        const content = readFileSync(configPath, 'utf8');
        return JSON.parse(content);
    } catch (error) {
        logError(`Failed to load module configuration: ${error.message}`);
        return {};
    }
}

function saveModuleConfig(config) {
    const configPath = getConfigPath();
    try {
        writeFileSync(configPath, JSON.stringify(config, null, 2));
        return true;
    } catch (error) {
        logError(`Failed to save module configuration: ${error.message}`);
        return false;
    }
}

function loadModuleManifest(moduleId) {
    const manifestPath = join(getModulesDirectory(), moduleId, 'module.json');
    if (!existsSync(manifestPath)) {
        return null;
    }

    try {
        const content = readFileSync(manifestPath, 'utf8');
        return JSON.parse(content);
    } catch (error) {
        logError(`Failed to load manifest for module '${moduleId}': ${error.message}`);
        return null;
    }
}

function getAvailableModules() {
    const modulesDir = getModulesDirectory();
    if (!existsSync(modulesDir)) {
        return [];
    }

    const modules = [];

    try {
        const entries = readdirSync(modulesDir, { withFileTypes: true });
        for (const entry of entries) {
            if (entry.isDirectory()) {
                const manifest = loadModuleManifest(entry.name);
                if (manifest) {
                    modules.push({
                        id: entry.name,
                        manifest
                    });
                }
            }
        }
    } catch (error) {
        logError(`Failed to scan modules directory: ${error.message}`);
    }

    return modules;
}

function getInstalledModules() {
    const srcTauriDir = join(projectRoot, 'src-tauri', 'src');
    const installedModules = [];

    // Check which modules are actually integrated in the source code
    const modules = getAvailableModules();

    for (const { id, manifest } of modules) {
        const moduleDir = join(srcTauriDir, 'modules', id);
        if (existsSync(moduleDir)) {
            installedModules.push({ id, manifest });
        }
    }

    return installedModules;
}

function copyModuleFiles(moduleId) {
    const moduleSource = join(getModulesDirectory(), moduleId);
    const srcTauriDir = join(projectRoot, 'src-tauri', 'src');
    const moduleTarget = join(srcTauriDir, 'modules', moduleId);

    if (!existsSync(moduleSource)) {
        throw new Error(`Module source directory not found: ${moduleSource}`);
    }

    // Create target directory
    mkdirSync(moduleTarget, { recursive: true });

    // Copy Rust source files if they exist
    const rustSrcDir = join(moduleSource, 'src');
    if (existsSync(rustSrcDir)) {
        const targetSrcDir = join(moduleTarget, 'src');
        mkdirSync(targetSrcDir, { recursive: true });
        cpSync(rustSrcDir, targetSrcDir, { recursive: true });
        logInfo(`Copied Rust source files for ${moduleId}`);
    }

    // Copy other relevant files
    const filesToCopy = ['mod.rs', 'lib.rs', 'handlers.rs', 'models.rs'];
    for (const file of filesToCopy) {
        const sourcePath = join(moduleSource, file);
        const targetPath = join(moduleTarget, file);
        if (existsSync(sourcePath)) {
            cpSync(sourcePath, targetPath);
            logInfo(`Copied ${file} for ${moduleId}`);
        }
    }
}

function removeModuleFiles(moduleId) {
    const srcTauriDir = join(projectRoot, 'src-tauri', 'src');
    const moduleTarget = join(srcTauriDir, 'modules', moduleId);

    if (existsSync(moduleTarget)) {
        rmSync(moduleTarget, { recursive: true, force: true });
        logInfo(`Removed module files for ${moduleId}`);
    }
}

function updateCargoToml(enabledModules) {
    const cargoPath = join(projectRoot, 'src-tauri', 'Cargo.toml');

    if (!existsSync(cargoPath)) {
        logError('Cargo.toml not found');
        return false;
    }

    try {
        let cargoContent = readFileSync(cargoPath, 'utf8');

        // Update the default features section
        const featureFlags = enabledModules.map(id => `"module-${id}"`).join(', ');
        const defaultFeatures = `default = [${featureFlags}]`;

        // Replace the existing default features line
        const featuresSectionRegex = /(\[features\][\s\S]*?default\s*=\s*\[)[^\]]*(\])/;

        if (featuresSectionRegex.test(cargoContent)) {
            cargoContent = cargoContent.replace(featuresSectionRegex, `$1${featureFlags}$2`);
        } else {
            logWarning('Could not find [features] section in Cargo.toml');
            return false;
        }

        writeFileSync(cargoPath, cargoContent);
        logSuccess(`Updated Cargo.toml with enabled modules: ${enabledModules.join(', ')}`);
        return true;
    } catch (error) {
        logError(`Failed to update Cargo.toml: ${error.message}`);
        return false;
    }
}

function updateModExports(enabledModules) {
    const libPath = join(projectRoot, 'src-tauri', 'src', 'lib.rs');

    if (!existsSync(libPath)) {
        logError('lib.rs not found');
        return false;
    }

    try {
        let libContent = readFileSync(libPath, 'utf8');

        // Remove existing module declarations
        libContent = libContent.replace(/mod modules;?\n?/g, '');

        // Add module declarations for enabled modules
        if (enabledModules.length > 0) {
            const moduleDeclarations = 'mod modules;\n';

            // Insert after the existing mod declarations
            const insertPoint = libContent.indexOf('use config::AppConfig;');
            if (insertPoint !== -1) {
                libContent = libContent.slice(0, insertPoint) + moduleDeclarations + libContent.slice(insertPoint);
            } else {
                libContent = moduleDeclarations + libContent;
            }
        }

        writeFileSync(libPath, libContent);
        logSuccess('Updated lib.rs module exports');
        return true;
    } catch (error) {
        logError(`Failed to update lib.rs: ${error.message}`);
        return false;
    }
}

function createModulesModFile(enabledModules) {
    const modulesDir = join(projectRoot, 'src-tauri', 'src', 'modules');
    const modFilePath = join(modulesDir, 'mod.rs');

    if (enabledModules.length === 0) {
        // Remove the modules directory if no modules are enabled
        if (existsSync(modulesDir)) {
            rmSync(modulesDir, { recursive: true, force: true });
        }
        return true;
    }

    // Create modules directory if it doesn't exist
    mkdirSync(modulesDir, { recursive: true });

    // Create mod.rs with module declarations
    const modContent = `//! Application modules
//!
//! This file is auto-generated by the module management CLI.
//! Do not edit manually.

${enabledModules.map(id => `#[cfg(feature = "module-${id}")]\npub mod ${id};`).join('\n\n')}

${enabledModules.map(id => `#[cfg(feature = "module-${id}")]\npub use ${id}::*;`).join('\n')}
`;

    try {
        writeFileSync(modFilePath, modContent);
        logSuccess('Created modules/mod.rs');
        return true;
    } catch (error) {
        logError(`Failed to create modules/mod.rs: ${error.message}`);
        return false;
    }
}

function listModules() {
    logInfo('Available modules:');
    console.log();

    const modules = getAvailableModules();
    const installedModules = getInstalledModules();
    const config = loadModuleConfig();

    if (modules.length === 0) {
        logWarning('No modules found in modules/ directory.');
        return;
    }

    for (const { id, manifest } of modules) {
        const moduleConfig = config[id];
        const enabled = moduleConfig?.enabled || false;
        const installed = installedModules.some(m => m.id === id);

        let status;
        if (enabled && installed) {
            status = colorize('ENABLED', 'green');
        } else if (installed) {
            status = colorize('INSTALLED', 'blue');
        } else {
            status = colorize('AVAILABLE', 'gray');
        }

        console.log(`${colorize('●', enabled ? 'green' : installed ? 'blue' : 'gray')} ${colorize(manifest.name, 'bold')} (${id}) - ${status}`);
        console.log(`  ${manifest.description}`);
        console.log(`  Version: ${manifest.version} | Category: ${manifest.category}`);

        if (manifest.dependencies.length > 0) {
            const deps = manifest.dependencies.map(dep => dep.module_id).join(', ');
            console.log(`  Dependencies: ${deps}`);
        }

        console.log();
    }
}

function installModule(moduleId) {
    const manifest = loadModuleManifest(moduleId);
    if (!manifest) {
        logError(`Module '${moduleId}' not found.`);
        return false;
    }

    const installedModules = getInstalledModules();
    if (installedModules.some(m => m.id === moduleId)) {
        logWarning(`Module '${moduleId}' is already installed.`);
        return true;
    }

    const config = loadModuleConfig();

    // Check dependencies
    for (const dep of manifest.dependencies) {
        if (!dep.optional) {
            const depInstalled = installedModules.some(m => m.id === dep.module_id);
            const depConfig = config[dep.module_id];

            if (!depInstalled || !depConfig?.enabled) {
                logError(`Cannot install '${moduleId}': dependency '${dep.module_id}' is not installed and enabled.`);
                logInfo(`Try installing the dependency first: npm run modules install ${dep.module_id}`);
                return false;
            }
        }
    }

    try {
        // Copy module files
        copyModuleFiles(moduleId);

        // Add to config as enabled
        if (!config[moduleId]) {
            config[moduleId] = { module_id: moduleId, enabled: false, config: {}, dependency_overrides: {} };
        }
        config[moduleId].enabled = true;

        // Update build files
        const enabledModules = Object.keys(config).filter(id => config[id].enabled);
        updateCargoToml(enabledModules);
        updateModExports(enabledModules);
        createModulesModFile(enabledModules);

        if (saveModuleConfig(config)) {
            logSuccess(`Installed and enabled module '${manifest.name}' (${moduleId})`);
            logInfo('Run `npm run build` to compile with the new module.');
            return true;
        }
    } catch (error) {
        logError(`Failed to install module '${moduleId}': ${error.message}`);
        return false;
    }

    return false;
}

function uninstallModule(moduleId) {
    const manifest = loadModuleManifest(moduleId);
    if (!manifest) {
        logError(`Module '${moduleId}' not found.`);
        return false;
    }

    const installedModules = getInstalledModules();
    if (!installedModules.some(m => m.id === moduleId)) {
        logWarning(`Module '${moduleId}' is not installed.`);
        return true;
    }

    if (!manifest.can_disable) {
        logError(`Module '${moduleId}' cannot be uninstalled (core module).`);
        return false;
    }

    const config = loadModuleConfig();
    const modules = getAvailableModules();

    // Check if other modules depend on this one
    for (const { id, manifest: otherManifest } of modules) {
        const otherConfig = config[id];
        if (otherConfig?.enabled && installedModules.some(m => m.id === id)) {
            for (const dep of otherManifest.dependencies) {
                if (!dep.optional && dep.module_id === moduleId) {
                    logError(`Cannot uninstall '${moduleId}': module '${id}' depends on it.`);
                    logInfo(`Uninstall '${id}' first, or install the dependency as optional.`);
                    return false;
                }
            }
        }
    }

    try {
        // Remove module files
        removeModuleFiles(moduleId);

        // Remove from config
        if (config[moduleId]) {
            delete config[moduleId];
        }

        // Update build files
        const enabledModules = Object.keys(config).filter(id => config[id].enabled);
        updateCargoToml(enabledModules);
        updateModExports(enabledModules);
        createModulesModFile(enabledModules);

        if (saveModuleConfig(config)) {
            logSuccess(`Uninstalled module '${manifest.name}' (${moduleId})`);
            logInfo('Run `npm run build` to compile without the module.');
            return true;
        }
    } catch (error) {
        logError(`Failed to uninstall module '${moduleId}': ${error.message}`);
        return false;
    }

    return false;
}

function configureModule(moduleId, key, value) {
    const manifest = loadModuleManifest(moduleId);
    if (!manifest) {
        logError(`Module '${moduleId}' not found.`);
        return false;
    }

    const config = loadModuleConfig();

    if (!config[moduleId]) {
        config[moduleId] = { module_id: moduleId, enabled: false, config: {}, dependency_overrides: {} };
    }

    // Validate configuration key exists in schema
    if (!manifest.config_schema[key]) {
        logError(`Configuration key '${key}' is not valid for module '${moduleId}'.`);
        logInfo(`Available keys: ${Object.keys(manifest.config_schema).join(', ')}`);
        return false;
    }

    const schema = manifest.config_schema[key];
    let parsedValue = value;

    // Parse value based on type
    try {
        switch (schema.field_type) {
            case 'boolean':
                parsedValue = value.toLowerCase() === 'true';
                break;
            case 'number':
                parsedValue = parseFloat(value);
                if (isNaN(parsedValue)) {
                    throw new Error('Invalid number');
                }
                break;
            case 'string':
                parsedValue = value;
                break;
            default:
                parsedValue = value;
        }
    } catch (error) {
        logError(`Invalid value for '${key}': expected ${schema.field_type}`);
        return false;
    }

    config[moduleId].config[key] = parsedValue;

    if (saveModuleConfig(config)) {
        logSuccess(`Set ${moduleId}.${key} = ${parsedValue}`);
        return true;
    }

    return false;
}

function showModuleInfo(moduleId) {
    const manifest = loadModuleManifest(moduleId);
    if (!manifest) {
        logError(`Module '${moduleId}' not found.`);
        return;
    }

    const config = loadModuleConfig();
    const moduleConfig = config[moduleId];

    console.log(colorize(`${manifest.name} (${moduleId})`, 'bold'));
    console.log(`Description: ${manifest.description}`);
    console.log(`Version: ${manifest.version}`);
    console.log(`Category: ${manifest.category}`);
    console.log(`Authors: ${manifest.authors.join(', ')}`);
    console.log(`License: ${manifest.license}`);
    console.log(`Can disable: ${manifest.can_disable ? 'Yes' : 'No'}`);
    console.log(`Status: ${moduleConfig?.enabled ? colorize('ENABLED', 'green') : colorize('DISABLED', 'gray')}`);

    if (manifest.dependencies.length > 0) {
        console.log('\\nDependencies:');
        for (const dep of manifest.dependencies) {
            const optional = dep.optional ? ' (optional)' : '';
            console.log(`  - ${dep.module_id} ${dep.version_req}${optional}`);
        }
    }

    if (manifest.commands.length > 0) {
        console.log('\\nProvided commands:');
        for (const cmd of manifest.commands) {
            console.log(`  - ${cmd}`);
        }
    }

    if (Object.keys(manifest.config_schema).length > 0) {
        console.log('\\nConfiguration schema:');
        for (const [key, schema] of Object.entries(manifest.config_schema)) {
            const currentValue = moduleConfig?.config[key];
            const defaultValue = schema.default !== undefined ? ` (default: ${schema.default})` : '';
            const required = schema.required ? ' *' : '';
            console.log(`  - ${key}: ${schema.field_type}${required}${defaultValue}`);
            console.log(`    ${schema.description}`);
            if (currentValue !== undefined) {
                console.log(`    Current value: ${currentValue}`);
            }
        }
    }

    console.log(`\\nTags: ${manifest.tags.join(', ')}`);
}

function syncModules() {
    const config = loadModuleConfig();
    const enabledModules = Object.keys(config).filter(id => config[id].enabled);

    logInfo('Syncing module configuration with build files...');

    updateCargoToml(enabledModules);
    updateModExports(enabledModules);
    createModulesModFile(enabledModules);

    logSuccess('Module sync completed.');
    logInfo('Run `npm run build` to compile with current module configuration.');
}

function showHelp() {
    console.log(colorize('EZ Tauri Module Management CLI', 'bold'));
    console.log();
    console.log('Usage:');
    console.log('  npm run modules <command> [options]');
    console.log();
    console.log('Commands:');
    console.log('  list                       List all available modules and their status');
    console.log('  install <module-id>        Install and enable a module');
    console.log('  uninstall <module-id>      Uninstall and disable a module');
    console.log('  info <module-id>           Show detailed information about a module');
    console.log('  config <module-id> <key> <value>  Set a configuration value for an installed module');
    console.log('  sync                       Sync module configuration with build files');
    console.log('  help                       Show this help message');
    console.log();
    console.log('Module Lifecycle:');
    console.log('  1. AVAILABLE  - Module exists in modules/ directory but is not installed');
    console.log('  2. INSTALLED  - Module files are copied to src-tauri and enabled in build');
    console.log('  3. CONFIGURED - Module settings can be customized via the config command');
    console.log();
    console.log('Examples:');
    console.log('  npm run modules list');
    console.log('  npm run modules install auth');
    console.log('  npm run modules uninstall cache');
    console.log('  npm run modules info database');
    console.log('  npm run modules config auth jwt_expiry_hours 48');
    console.log('  npm run modules sync');
    console.log();
    console.log('Note: After installing/uninstalling modules, run "npm run build" to recompile.');
}

// Main CLI logic
function main() {
    const args = process.argv.slice(2);
    const command = args[0];

    switch (command) {
        case 'list':
            listModules();
            break;

        case 'install':
            if (args.length < 2) {
                logError('Module ID required. Usage: npm run modules install <module-id>');
                process.exit(1);
            }
            installModule(args[1]);
            break;

        case 'uninstall':
            if (args.length < 2) {
                logError('Module ID required. Usage: npm run modules uninstall <module-id>');
                process.exit(1);
            }
            uninstallModule(args[1]);
            break;

        case 'info':
            if (args.length < 2) {
                logError('Module ID required. Usage: npm run modules info <module-id>');
                process.exit(1);
            }
            showModuleInfo(args[1]);
            break;

        case 'config':
            if (args.length < 4) {
                logError('Usage: npm run modules config <module-id> <key> <value>');
                process.exit(1);
            }
            configureModule(args[1], args[2], args[3]);
            break;

        case 'sync':
            syncModules();
            break;

        case 'help':
        case '--help':
        case '-h':
            showHelp();
            break;

        default:
            if (command) {
                logError(`Unknown command: ${command}`);
            }
            showHelp();
            process.exit(1);
    }
}

// Ensure modules directory exists
const modulesDir = getModulesDirectory();
if (!existsSync(modulesDir)) {
    mkdirSync(modulesDir, { recursive: true });
}

main();