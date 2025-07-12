#!/bin/bash

# Copyright 2025 Roberto Antunes
# Build script for Lingo Database JavaScript/TypeScript bindings

set -e

echo "🚀 Building Lingo Database JavaScript/TypeScript bindings..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required tools are installed
check_requirements() {
    print_status "Checking requirements..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found. Please install Rust: https://rustup.rs/"
        exit 1
    fi
    
    if ! command -v wasm-pack &> /dev/null; then
        print_error "wasm-pack not found. Installing..."
        cargo install wasm-pack
    fi
    
    if ! command -v node &> /dev/null; then
        print_error "Node.js not found. Please install Node.js: https://nodejs.org/"
        exit 1
    fi
    
    print_success "All requirements satisfied"
}

# Build the Rust WASM module
build_wasm() {
    print_status "Building WebAssembly module..."
    
    # Build for Node.js
    print_status "Building for Node.js..."
    wasm-pack build --target nodejs --out-dir js/pkg --release
    
    # Build for web browsers
    print_status "Building for web browsers..."
    wasm-pack build --target web --out-dir js/pkg-web --release
    
    # Build for bundlers (Webpack, Rollup)
    print_status "Building for bundlers..."
    wasm-pack build --target bundler --out-dir js/pkg-bundler --release
    
    print_success "WebAssembly modules built successfully"
}

# Setup Node.js package
setup_nodejs() {
    print_status "Setting up Node.js package..."
    
    cd js
    
    if [ ! -f "package-lock.json" ]; then
        print_status "Installing Node.js dependencies..."
        npm install
    else
        print_status "Updating Node.js dependencies..."
        npm ci
    fi
    
    print_success "Node.js package setup complete"
    cd ..
}

# Build TypeScript
build_typescript() {
    print_status "Building TypeScript..."
    
    cd js
    
    # Install TypeScript if not present
    if ! command -v npx tsc &> /dev/null; then
        npm install -g typescript
    fi
    
    # Compile TypeScript
    npx tsc
    
    print_success "TypeScript compiled successfully"
    cd ..
}

# Run tests
run_tests() {
    print_status "Running tests..."
    
    cd js
    
    # Run JavaScript tests
    if [ -f "jest.config.js" ] || [ -f "package.json" ]; then
        npm test
        print_success "JavaScript tests passed"
    else
        print_warning "No JavaScript tests found"
    fi
    
    cd ..
    
    # Run Rust tests
    print_status "Running Rust tests..."
    cargo test --features wasm
    print_success "Rust tests passed"
}

# Create example database
create_example_db() {
    print_status "Creating example database..."
    
    # Build the example if it exists
    if [ -f "examples/build_db.rs" ]; then
        cargo run --example build_db
        print_success "Example database created"
    else
        print_warning "No database builder example found"
    fi
}

# Package for distribution
package_dist() {
    print_status "Packaging for distribution..."
    
    cd js
    
    # Create dist directory structure
    mkdir -p dist
    
    # Copy built files
    if [ -d "pkg" ]; then
        cp -r pkg/* dist/
    fi
    
    # Copy package files
    cp package.json dist/
    cp README.md dist/
    cp ../LICENSE dist/
    
    print_success "Package ready for distribution in js/dist/"
    cd ..
}

# Main build process
main() {
    echo "=================================="
    echo "🚀 Lingo Database JS/TS Builder"
    echo "=================================="
    echo
    
    check_requirements
    echo
    
    build_wasm
    echo
    
    setup_nodejs
    echo
    
    build_typescript
    echo
    
    create_example_db
    echo
    
    if [ "$1" = "--test" ]; then
        run_tests
        echo
    fi
    
    if [ "$1" = "--package" ]; then
        package_dist
        echo
    fi
    
    print_success "🎉 Build completed successfully!"
    echo
    echo "📦 Built packages:"
    echo "  - js/pkg/         (Node.js)"
    echo "  - js/pkg-web/     (Web browsers)"
    echo "  - js/pkg-bundler/ (Webpack/Rollup)"
    echo "  - js/dist/        (TypeScript compiled)"
    echo
    echo "🚀 Next steps:"
    echo "  1. cd js && npm test              # Run tests"
    echo "  2. cd js && npm run example       # Try examples"
    echo "  3. cd js && npm publish           # Publish to npm"
    echo
}

# Handle script arguments
case "$1" in
    --help|-h)
        echo "Usage: $0 [OPTIONS]"
        echo
        echo "Options:"
        echo "  --test      Run tests after building"
        echo "  --package   Create distribution package"
        echo "  --help      Show this help message"
        exit 0
        ;;
    *)
        main "$1"
        ;;
esac