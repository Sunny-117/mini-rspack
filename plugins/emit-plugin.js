/**
 * EmitPlugin - 生成 assets.md 文件，列出所有资源
 */
class EmitPlugin {
  constructor(options = {}) {
    this.options = options;
  }

  apply(compiler) {
    // 注册 emit 钩子
    compiler.hooks.emit.tap('EmitPlugin', (assets) => {
      console.log('EmitPlugin: Processing assets...');

      // 获取所有资源的列表
      const assetsList = Object.keys(assets);
      console.log('EmitPlugin: Found assets:', assetsList);

      // 生成 Markdown 格式的资源列表
      let content = '# Assets\n\n';
      assetsList.sort().forEach(asset => {
        content += `- ${asset}\n`;
      });

      // 添加资源列表到输出
      console.log('EmitPlugin: Adding assets.md file');
      assets['assets.md'] = content;
    });
  }
}

module.exports = EmitPlugin;
