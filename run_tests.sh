#!/bin/bash

# 🧪 Dazno Umbrel - Test Runner Script
# Comprehensive test suite for API interactions and system validation

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Test configuration
RUST_LOG=${RUST_LOG:-"info"}
TEST_THREADS=${TEST_THREADS:-"4"}
ENABLE_REAL_API_TESTS=${ENABLE_REAL_API_TESTS:-"false"}

echo -e "${BLUE}🧪 Dazno Umbrel Test Suite${NC}"
echo -e "${BLUE}==============================${NC}"
echo ""

# Function to run a test category
run_test_category() {
    local category=$1
    local description=$2
    local extra_args=$3
    
    echo -e "${CYAN}🔍 Running $description...${NC}"
    
    if RUST_LOG=$RUST_LOG cargo test $category --test $category $extra_args; then
        echo -e "${GREEN}✅ $description completed successfully${NC}"
        return 0
    else
        echo -e "${RED}❌ $description failed${NC}"
        return 1
    fi
}

# Function to run unit tests
run_unit_tests() {
    echo -e "${PURPLE}📋 Unit Tests${NC}"
    echo -e "${PURPLE}==============${NC}"
    
    echo -e "${CYAN}🔍 Running MCP Client unit tests...${NC}"
    if RUST_LOG=$RUST_LOG cargo test api::mcp_client::tests --lib; then
        echo -e "${GREEN}✅ MCP Client tests passed${NC}"
    else
        echo -e "${RED}❌ MCP Client tests failed${NC}"
        return 1
    fi
    
    echo -e "${CYAN}🔍 Running Local Lightning Client unit tests...${NC}"
    if RUST_LOG=$RUST_LOG cargo test api::local_lightning_client --lib; then
        echo -e "${GREEN}✅ Local Lightning Client tests passed${NC}"
    else
        echo -e "${RED}❌ Local Lightning Client tests failed${NC}"
        return 1
    fi
    
    echo ""
}

# Function to run integration tests
run_integration_tests() {
    echo -e "${PURPLE}🔗 Integration Tests${NC}"
    echo -e "${PURPLE}===================${NC}"
    
    if run_test_category "integration_tests" "Integration Tests" "-- --test-threads=$TEST_THREADS"; then
        return 0
    else
        return 1
    fi
    
    echo ""
}

# Function to run performance tests
run_performance_tests() {
    echo -e "${PURPLE}⚡ Performance Tests${NC}"
    echo -e "${PURPLE}===================${NC}"
    
    if run_test_category "performance_tests" "Performance Tests" "-- --test-threads=1"; then
        return 0
    else
        return 1
    fi
    
    echo ""
}

# Function to run mock API tests
run_mock_api_tests() {
    echo -e "${PURPLE}🎭 Mock API Tests${NC}"
    echo -e "${PURPLE}=================${NC}"
    
    if run_test_category "mock_api_server" "Mock API Server Tests" "-- --test-threads=$TEST_THREADS"; then
        return 0
    else
        return 1
    fi
    
    echo ""
}

# Function to run real API tests (if enabled)
run_real_api_tests() {
    if [ "$ENABLE_REAL_API_TESTS" = "true" ]; then
        echo -e "${PURPLE}🌐 Real API Tests${NC}"
        echo -e "${PURPLE}=================${NC}"
        echo -e "${YELLOW}⚠️  Running tests against real api.dazno.de${NC}"
        echo -e "${YELLOW}   These tests may fail if the API is unavailable${NC}"
        echo ""
        
        # Set environment for real API tests
        export ENABLE_REAL_API_TESTS=true
        
        if RUST_LOG=$RUST_LOG cargo test test_real_api --test integration_tests; then
            echo -e "${GREEN}✅ Real API tests passed${NC}"
        else
            echo -e "${YELLOW}⚠️  Real API tests failed (this may be expected)${NC}"
        fi
        echo ""
    else
        echo -e "${YELLOW}ℹ️  Real API tests disabled (set ENABLE_REAL_API_TESTS=true to enable)${NC}"
        echo ""
    fi
}

# Function to generate test report
generate_test_report() {
    echo -e "${BLUE}📊 Test Coverage Report${NC}"
    echo -e "${BLUE}=======================${NC}"
    
    # Run cargo tarpaulin if available
    if command -v cargo-tarpaulin &> /dev/null; then
        echo -e "${CYAN}🔍 Generating code coverage report...${NC}"
        cargo tarpaulin --out Html --output-dir target/coverage || {
            echo -e "${YELLOW}⚠️  Coverage report generation failed${NC}"
        }
    else
        echo -e "${YELLOW}ℹ️  Install cargo-tarpaulin for coverage reports: cargo install cargo-tarpaulin${NC}"
    fi
    
    echo ""
}

# Function to run benchmarks
run_benchmarks() {
    echo -e "${PURPLE}🏎️  Benchmarks${NC}"
    echo -e "${PURPLE}=============${NC}"
    
    if cargo bench --no-run &> /dev/null; then
        echo -e "${CYAN}🔍 Running benchmarks...${NC}"
        cargo bench || {
            echo -e "${YELLOW}⚠️  Benchmarks failed or not available${NC}"
        }
    else
        echo -e "${YELLOW}ℹ️  No benchmarks configured${NC}"
    fi
    
    echo ""
}

# Function to validate code quality
run_code_quality_checks() {
    echo -e "${PURPLE}🔍 Code Quality Checks${NC}"
    echo -e "${PURPLE}======================${NC}"
    
    echo -e "${CYAN}🔍 Running cargo clippy...${NC}"
    if cargo clippy --all-targets --all-features -- -D warnings; then
        echo -e "${GREEN}✅ Clippy checks passed${NC}"
    else
        echo -e "${RED}❌ Clippy checks failed${NC}"
        return 1
    fi
    
    echo -e "${CYAN}🔍 Running cargo fmt check...${NC}"
    if cargo fmt -- --check; then
        echo -e "${GREEN}✅ Format checks passed${NC}"
    else
        echo -e "${RED}❌ Format checks failed${NC}"
        echo -e "${YELLOW}💡 Run 'cargo fmt' to fix formatting${NC}"
        return 1
    fi
    
    echo ""
}

# Function to test documentation
test_documentation() {
    echo -e "${PURPLE}📚 Documentation Tests${NC}"
    echo -e "${PURPLE}=====================${NC}"
    
    echo -e "${CYAN}🔍 Testing documentation examples...${NC}"
    if cargo test --doc; then
        echo -e "${GREEN}✅ Documentation tests passed${NC}"
    else
        echo -e "${RED}❌ Documentation tests failed${NC}"
        return 1
    fi
    
    echo ""
}

# Main test execution
main() {
    local failed_tests=0
    
    echo -e "${BLUE}Configuration:${NC}"
    echo -e "  RUST_LOG: $RUST_LOG"
    echo -e "  TEST_THREADS: $TEST_THREADS"
    echo -e "  ENABLE_REAL_API_TESTS: $ENABLE_REAL_API_TESTS"
    echo ""
    
    # Build first
    echo -e "${CYAN}🔨 Building project...${NC}"
    if cargo build --all-targets; then
        echo -e "${GREEN}✅ Build successful${NC}"
    else
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    fi
    echo ""
    
    # Run test categories
    if ! run_unit_tests; then
        ((failed_tests++))
    fi
    
    if ! run_integration_tests; then
        ((failed_tests++))
    fi
    
    if ! run_performance_tests; then
        ((failed_tests++))
    fi
    
    if ! run_mock_api_tests; then
        ((failed_tests++))
    fi
    
    # Optional tests
    run_real_api_tests
    
    if ! run_code_quality_checks; then
        ((failed_tests++))
    fi
    
    if ! test_documentation; then
        ((failed_tests++))
    fi
    
    # Optional reports
    generate_test_report
    run_benchmarks
    
    # Final report
    echo -e "${BLUE}📋 Test Summary${NC}"
    echo -e "${BLUE}===============${NC}"
    
    if [ $failed_tests -eq 0 ]; then
        echo -e "${GREEN}🎉 All test categories passed successfully!${NC}"
        echo -e "${GREEN}✅ Ready for deployment${NC}"
        exit 0
    else
        echo -e "${RED}❌ $failed_tests test categories failed${NC}"
        echo -e "${RED}🚨 Please fix issues before deployment${NC}"
        exit 1
    fi
}

# Parse command line arguments
case "${1:-all}" in
    "unit")
        run_unit_tests
        ;;
    "integration")
        run_integration_tests
        ;;
    "performance")
        run_performance_tests
        ;;
    "mock")
        run_mock_api_tests
        ;;
    "real-api")
        ENABLE_REAL_API_TESTS=true run_real_api_tests
        ;;
    "quality")
        run_code_quality_checks
        ;;
    "docs")
        test_documentation
        ;;
    "coverage")
        generate_test_report
        ;;
    "bench")
        run_benchmarks
        ;;
    "all"|*)
        main
        ;;
esac