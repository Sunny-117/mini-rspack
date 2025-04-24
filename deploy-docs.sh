#!/bin/bash

# 设置颜色
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 显示带颜色的消息
info() {
  echo -e "${GREEN}INFO: $1${NC}"
}

warn() {
  echo -e "${YELLOW}WARN: $1${NC}"
}

error() {
  echo -e "${RED}ERROR: $1${NC}"
}

# 检查是否安装了必要的工具
check_dependencies() {
  info "检查依赖..."
  
  if ! command -v git &> /dev/null; then
    error "未找到git，请安装git"
    exit 1
  fi
  
  if ! command -v pnpm &> /dev/null; then
    warn "未找到pnpm，尝试使用npm..."
    if ! command -v npm &> /dev/null; then
      error "未找到npm，请安装npm或pnpm"
      exit 1
    fi
    USE_NPM=true
  else
    USE_NPM=false
  fi
}

# 保存当前分支
save_current_branch() {
  CURRENT_BRANCH=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [ $? -ne 0 ]; then
    CURRENT_BRANCH="detached-head"
  fi
  info "当前分支: $CURRENT_BRANCH"
}

# 构建文档
build_docs() {
  info "构建文档..."
  
  cd docs
  
  if [ "$USE_NPM" = true ]; then
    npm install
    npm run docs:build
  else
    pnpm install
    pnpm docs:build
  fi
  
  if [ $? -ne 0 ]; then
    error "文档构建失败"
    cd ..
    exit 1
  fi
  
  cd ..
  info "文档构建成功"
}

# 部署到gh-pages分支
deploy_to_gh_pages() {
  info "部署到gh-pages分支..."
  
  # 检查gh-pages分支是否存在
  if git show-ref --verify --quiet refs/heads/gh-pages; then
    info "gh-pages分支已存在，将被更新"
  else
    info "创建gh-pages分支"
    git checkout --orphan gh-pages
    git rm -rf .
    git commit --allow-empty -m "Initial gh-pages branch"
    git checkout $CURRENT_BRANCH
  fi
  
  # 创建临时目录
  TEMP_DIR=$(mktemp -d)
  info "创建临时目录: $TEMP_DIR"
  
  # 复制构建文件到临时目录
  cp -r docs/.vitepress/dist/* $TEMP_DIR
  
  # 切换到gh-pages分支
  git checkout gh-pages
  
  # 清空当前目录（保留.git）
  find . -maxdepth 1 ! -name .git ! -name . -exec rm -rf {} \;
  
  # 复制构建文件到当前目录
  cp -r $TEMP_DIR/* .
  
  # 添加.nojekyll文件（防止GitHub Pages忽略以下划线开头的文件）
  touch .nojekyll
  
  # 添加所有文件
  git add -A
  
  # 提交更改
  git commit -m "Deploy documentation $(date)"
  
  # 推送到远程仓库
  info "推送到远程仓库..."
  git push origin gh-pages
  
  if [ $? -ne 0 ]; then
    error "推送失败，请检查你的权限"
    # 清理并返回原始分支
    rm -rf $TEMP_DIR
    git checkout $CURRENT_BRANCH
    exit 1
  fi
  
  # 清理并返回原始分支
  rm -rf $TEMP_DIR
  git checkout $CURRENT_BRANCH
  
  info "部署成功！文档已发布到gh-pages分支"
  info "访问 https://$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/' | sed 's/\//.github.io\//')/mini-rspack/ 查看文档"
}

# 主函数
main() {
  info "开始部署文档..."
  
  # 检查是否在git仓库根目录
  if [ ! -d ".git" ]; then
    error "请在git仓库根目录运行此脚本"
    exit 1
  fi
  
  # 检查依赖
  check_dependencies
  
  # 保存当前分支
  save_current_branch
  
  # 检查工作目录是否干净
  if [ -n "$(git status --porcelain)" ]; then
    warn "工作目录不干净，建议先提交或暂存更改"
    read -p "是否继续？(y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
      info "已取消部署"
      exit 0
    fi
  fi
  
  # 构建文档
  build_docs
  
  # 部署到gh-pages分支
  deploy_to_gh_pages
  
  info "文档部署完成！"
}

# 执行主函数
main
