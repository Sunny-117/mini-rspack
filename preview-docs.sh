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

# 构建并预览文档
build_and_preview() {
  info "构建并预览文档..."
  
  cd docs
  
  if [ "$USE_NPM" = true ]; then
    npm install
    npm run docs:build
    
    # 使用npx启动预览服务器
    info "启动预览服务器..."
    npx vitepress preview --port 5173
  else
    pnpm install
    pnpm docs:build
    
    # 使用pnpm启动预览服务器
    info "启动预览服务器..."
    pnpm exec vitepress preview --port 5173
  fi
  
  if [ $? -ne 0 ]; then
    error "预览服务器启动失败"
    cd ..
    exit 1
  fi
}

# 主函数
main() {
  info "开始预览文档..."
  
  # 检查依赖
  check_dependencies
  
  # 构建并预览文档
  build_and_preview
}

# 执行主函数
main
