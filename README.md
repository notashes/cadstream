# CAD Stream Processor

A high-performance Rust library for streaming, processing, and visualizing CAD files in real-time with Rerun. This project demonstrates real-time CAD file parsing, professional 3D visualization, and file watching capabilities.


## 🚀 Features

- **🌐 Rerun Integration** - Professional data visualization with web/desktop viewer
- **⚡ Real-time STL parsing** - Both ASCII and binary STL formats
- **👀 File watching** - Automatically detects and loads new STL files
- **🖱️ Interactive controls** - Mouse-based camera orbit and zoom in Rerun viewer
- **🌍 Cross-platform** - Runs on Windows, macOS, and Linux
- **📊 Rich metadata** - Model dimensions, triangle counts, and bounding boxes

## 🛠️ Technical Stack

### Core Engine
- **Rust** - Systems programming language for performance
- **Tokio** - Async runtime for file operations  
- **Notify** - File system watching
- **Glam** - 3D mathematics library

### Visualization
- **🌐 Rerun** - Professional data visualization platform
- **gRPC streaming** - Real-time data transmission
- **Web/Desktop viewer** - Cross-platform visualization

## 📋 Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Python 3.8+ with pip (for Rerun viewer)

## 🏃‍♂️ Quick Start

### 🌐 **Rerun Visualization**

```bash
git clone <repository-url>
cd cadstream

# Install Rerun viewer
pip install rerun-sdk

# Start the CAD stream processor with default parser
cargo run

# Or use specific STL parser implementation
cargo run --features stl-io-parser --no-default-features
cargo run --features custom-stl-parser --no-default-features
```

### 🔧 **Parser Selection**

This project supports multiple STL parsing implementations:

- **`custom-stl-parser`** (default) - Our own high-performance STL parser
- **`stl-io-parser`** - Using the established `stl_io` crate

Choose your parser with feature flags:

```bash
# Use our custom parser (default)
cargo run

# Use stl_io crate
cargo run --features stl-io-parser --no-default-features

# Build with specific parser
cargo build --features stl-io-parser --no-default-features
```

### 🚀 **Connection Process**
1. **Start the server**: Run `cargo run` to start the gRPC server
2. **Connect viewer**: In a new terminal, run `rerun --connect rerun+http://127.0.0.1:9876/proxy`
3. **View data**: The Rerun viewer will automatically display CAD models

### 🎯 **What Happens**
1. **File Discovery**: Automatically scans for STL files
2. **Demo Creation**: Creates `test_cube.stl` if none exist
3. **Real-time Processing**: Parses and streams CAD data
4. **gRPC Server**: Hosts data on port 9876
5. **Live Updates**: Watches directory for new files
6. **Professional Visualization**: Rich 3D viewer with metadata

## 🎮 Viewer Controls

The Rerun viewer provides professional 3D interaction:

| Action | Control |
|--------|---------|
| Orbit camera | Left mouse button + drag |
| Pan view | Middle mouse button + drag |
| Zoom | Mouse wheel |
| Reset view | Double-click on object |
| Timeline | Navigate through file changes |

## 📁 Supported Formats

Currently supported:
- **STL** (STereoLithography)
  - ASCII format
  - Binary format
  - Error handling for malformed files

*Future formats planned: OBJ, PLY, DXF*

## 🔧 Architecture

```
┌─────────────────┐     ┌──────────────┐     ┌─────────────┐
│   File Watcher  │───▶ │  STL Parser  │───▶ │ CAD Model   │
│                 │     │              │     │             │
│ - notify crate  │     │ - ASCII/Bin  │     │ - Triangles │
│ - Real-time     │     │ - Validation │     │ - Bounds    │
│ - Auto-reload   │     │ - Precision  │     │ - Metadata  │
└─────────────────┘     └──────────────┘     └─────────────┘
                                                    │
                                                    ▼
┌─────────────────┐     ┌──────────────┐     ┌─────────────┐
│   Rerun Stream  │◀─── │ Data Logging │◀─── │  Renderer   │
│                 │     │              │     │             │
│ - gRPC Server   │     │ - Mesh Data  │     │ - Real-time │
│ - Web Viewer    │     │ - Metadata   │     │ - Streaming │
│ - Timeline      │     │ - Bounds     │     │ - Updates   │
└─────────────────┘     └──────────────┘     └─────────────┘
```

## 🧪 Testing

Run the included tests:

```bash
# Unit tests
cargo test

# Test with sample files
cargo run
# Drop STL files into the directory to test parsing

# Run benchmarks (test both parsers)
cargo bench
cargo bench --features stl-io-parser --no-default-features
```

## 📊 Performance Metrics

Current performance characteristics:
- **Parsing**: ~10MB/s for ASCII STL, ~50MB/s for binary STL (custom parser)
- **Parsing**: Performance varies with `stl_io` parser (benchmarks available)
- **Streaming**: Real-time gRPC transmission to Rerun
- **Memory**: ~2x file size peak memory usage
- **Startup**: <100ms from launch to data streaming

### 🏗️ **Parser Architecture**

**Extensible Design for Multiple CAD Formats:**
- **`FileParser` trait**: Common interface for all CAD format parsers
- **`ParserFactory`**: Automatically selects parser based on file format and enabled features
- **Format detection**: Automatic file format identification by extension
- **Feature flags**: Choose parser implementation at compile time
- **Future-ready**: Designed to easily add new formats (OBJ, PLY, STEP, etc.)

**Current STL Parser Options:**
- **`custom-stl-parser`** (default): Our own optimized implementation
- **`stl-io-parser`**: Using the established `stl_io` crate

## 🚧 Development Roadmap

### Phase 1: Core Infrastructure ✅
- [x] Basic STL parsing (ASCII/Binary)
- [x] Rerun integration and streaming
- [x] File watching system
- [x] Professional 3D visualization

### Phase 2: Enhanced Features (Next)
- [ ] Additional CAD formats (OBJ, PLY)
- [ ] Precision tracking and validation
- [ ] Performance profiling and metrics
- [ ] Material and color support

### Phase 3: Advanced Features (Future)
- [ ] Custom intermediate format
- [ ] Multi-file assembly support
- [ ] Advanced Rerun features (annotations, measurements)
- [ ] Collaborative features

## 🤝 Contributing

This project demonstrates:
- **File format parsing** - Robust handling of CAD formats
- **Real-time processing** - Streaming and live updates
- **Professional visualization** - Rerun integration
- **Systems programming** - Memory-efficient Rust code
- **Cross-platform development** - Works on all major OS

Perfect for portfolios targeting:
- **CAD/Engineering software companies**
- **3D graphics and visualization**
- **High-performance systems programming**
- **Real-time data processing**

## 📝 License

This project is open source and available under the MIT License.

## 🐛 Known Issues

- Large files (>50MB) may take several seconds to process
- Only one file is displayed at a time (multi-file support planned)
- Requires Python environment for Rerun viewer

## 💡 Tips

### Performance & Usage
- For best performance, use binary STL files when possible
- Both parsers handle malformed files gracefully with detailed error messages
- Use `RUST_LOG=debug cargo run` for detailed parsing information
- Rerun viewer provides timeline navigation for file changes
- Connect multiple viewers to the same server for collaborative viewing

### Parser Selection
- **Custom parser**: Generally faster, more control, demonstrates parsing skills
- **stl_io parser**: Battle-tested, fewer edge cases, external dependency
- Compare performance: `cargo bench` vs `cargo bench --features stl-io-parser --no-default-features`
- The console output shows which parser is being used: `(using Custom STL Parser)` or `(using stl_io Parser)`

### Future Development
- Adding new formats is as simple as implementing the `FileParser` trait
- Feature flags allow users to choose only the parsers they need
- The architecture supports mixed format workflows (future: STL + OBJ + PLY) 