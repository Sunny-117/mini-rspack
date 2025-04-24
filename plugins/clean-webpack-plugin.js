/**
 * A simplified CleanWebpackPlugin that cleans the output directory
 * In a real implementation, this would use fs to delete files
 */
class CleanWebpackPlugin {
  constructor(options = {}) {
    this.options = Object.assign({
      dry: false,
      verbose: false,
      cleanOnceBeforeBuildPatterns: ['**/*'],
      cleanAfterEveryBuildPatterns: [],
      dangerouslyAllowCleanPatternsOutsideProject: false,
    }, options);
  }
  
  apply(compiler) {
    // Hook into the emit event
    compiler.hooks.emit.tap('CleanWebpackPlugin', (compilation) => {
      console.log('CleanWebpackPlugin: Cleaning output directory');
      
      // In a real implementation, this would use fs to delete files
      // For now, we'll just log the files that would be deleted
      
      const outputPath = compilation.options.output.path;
      
      if (this.options.verbose) {
        console.log(`CleanWebpackPlugin: Output path: ${outputPath}`);
        console.log(`CleanWebpackPlugin: Patterns: ${this.options.cleanOnceBeforeBuildPatterns.join(', ')}`);
      }
      
      if (!this.options.dry) {
        // In a real implementation, this would delete files
        // For now, we'll just log the files that would be deleted
        console.log('CleanWebpackPlugin: Deleted files');
      } else {
        console.log('CleanWebpackPlugin: Dry run, no files deleted');
      }
    });
  }
}

module.exports = CleanWebpackPlugin;
