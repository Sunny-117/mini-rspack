export default {
  base: '/mini-rspack/',
  title: 'mini-rspack',
  description: 'A simplified implementation of webpack/rspack bundler using Rust for educational purposes',
  themeConfig: {
    logo: '/logo.png',
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/' },
      { text: 'API', link: '/api/' },
      { text: 'Advanced', link: '/advanced/' },
      { text: 'GitHub', link: 'https://github.com/Sunny-117/mini-rspack' }
    ],
    sidebar: {
      '/guide/': [
        {
          text: 'Guide',
          items: [
            { text: 'Introduction', link: '/guide/' },
            { text: 'Getting Started', link: '/guide/getting-started' },
            { text: 'Configuration', link: '/guide/configuration' },
            { text: 'Plugins', link: '/guide/plugins' },
            { text: 'Loaders', link: '/guide/loaders' },
            { text: 'Architecture', link: '/guide/architecture' },
          ]
        }
      ],
      '/api/': [
        {
          text: 'API',
          items: [
            { text: 'Overview', link: '/api/' },
            { text: 'Compiler', link: '/api/compiler' },
            { text: 'Compilation', link: '/api/compilation' },
            { text: 'Module', link: '/api/module' },
            { text: 'Hooks', link: '/api/hooks' },
            { text: 'Loader', link: '/api/loader' },
            { text: 'Plugin', link: '/api/plugin' },
          ]
        }
      ],
      '/advanced/': [
        {
          text: 'Advanced',
          items: [
            { text: 'Technical Implementation', link: '/advanced/' },
            { text: 'Testing', link: '/advanced/testing' },
            { text: 'Performance', link: '/advanced/performance' },
          ]
        }
      ]
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Sunny-117/mini-rspack' }
    ],
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2023-present'
    }
  },
  locales: {
    root: {
      label: 'English',
      lang: 'en'
    },
    zh: {
      label: '中文',
      lang: 'zh-CN',
      link: '/zh/',
      themeConfig: {
        nav: [
          { text: '首页', link: '/zh/' },
          { text: '指南', link: '/zh/guide/' },
          { text: 'API', link: '/zh/api/' },
          { text: 'GitHub', link: 'https://github.com/Sunny-117/mini-rspack' }
        ],
        sidebar: {
          '/zh/guide/': [
            {
              text: '指南',
              items: [
                { text: '介绍', link: '/zh/guide/' },
                { text: '快速开始', link: '/zh/guide/getting-started' },
                { text: '配置', link: '/zh/guide/configuration' },
                { text: '插件', link: '/zh/guide/plugins' },
                { text: '加载器', link: '/zh/guide/loaders' },
              ]
            }
          ],
          '/zh/api/': [
            {
              text: 'API',
              items: [
                { text: '概述', link: '/zh/api/' },
                { text: '编译器', link: '/zh/api/compiler' },
                { text: '编译', link: '/zh/api/compilation' },
                { text: '模块', link: '/zh/api/module' },
                { text: '钩子', link: '/zh/api/hooks' },
              ]
            }
          ]
        },
        footer: {
          message: '基于 MIT 许可发布',
          copyright: '版权所有 © 2023-至今'
        }
      }
    }
  }
}
