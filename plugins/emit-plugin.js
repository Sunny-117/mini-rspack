class EmitPlugin {
  apply(compiler) {
    compiler.hooks.emit.tap('emit', (assets) => {
      assets['assets.md'] = Object.keys(assets).join('\n');
      console.log('This is triggered before emitting files');
    });
  }
}

module.exports = EmitPlugin;
