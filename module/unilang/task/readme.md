# Task Management System

| Directory | Responsibility |
|-----------|---------------|
| completed/ | Completed task files |
| unverified/ | Tasks awaiting verification |
| cancelled/ | Cancelled tasks |
| bug/ | Bug reports and tracking |
| actors/ | Actor registry for task execution |
| action_plan/ | Per-actor action plans |
| decisions/ | Decision records |

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | State | Executor | Task | Purpose |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|----------|------|-------------|
| 1 | 104 | 0 | 8 | 9 | 9 | 0 | ✅ (Completed) | claude-opus-4-6 | [104_doc_normalization_and_entity_expansion](./completed/104_doc_normalization_and_entity_expansion.md) | [admin] Normalize 63 section headings, create type/ entity (4 instances), invariant/006, add Sources/Tests traceability |
| 2 | 103 | 0 | 7 | 9 | 9 | 0 | ✅ (Completed) | claude-sonnet-4-6 | [103_fix_help_self_referential_visibility](./completed/103_fix_help_self_referential_visibility.md) | Fix .help visible in its own listing — `hidden_from_list: false` → `true` in dynamic.rs; closes BUG-102 |
| 3 | 098 | 0 | 9 | 4 | 6 | 0 | ✅ (Completed) | claude-sonnet-4-6 | [098_implement_optional_dep_pattern](./completed/098_implement_optional_dep_pattern.md) | Make all library crate deps optional; wire `enabled` feature with `dep:name` syntax for no-op-when-disabled |
| 4 | 097 | 0 | 6 | 9 | 9 | 0 | ✅ (Completed) | claude-sonnet-4-6 | [097_fix_dep_version_format](./completed/097_fix_dep_version_format.md) | Rewrite workspace Cargo.toml version strings to `^X.Y` / `=X.Y.Z` format per invariant 004 R1 |
| 5 | 096 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | claude-sonnet-4-6 | [096_add_marker_types](./completed/096_add_marker_types.md) | Add ShellArgv/ReplInput newtypes and parse_cli/parse_repl type-safe entry points |
| 6 | 095 | 0 | 9 | 5 | 7 | 0 | ✅ (Completed) | claude-sonnet-4-6 | [095_rename_parse_single_instruction](./completed/095_rename_parse_single_instruction.md) | Rename parse_single_instruction → parse_repl_input with deprecation shim across 302 call sites |
| 7 | 092 | 0 | 10 | 8 | 5 | 0 | ✅ (Completed) | N/A | [092_fix_incorrect_file_path_documentation](./completed/092_fix_incorrect_file_path_documentation.md) | Fix widespread incorrect documentation claiming unilang cannot parse file paths - actually works with :: syntax |
| 8 | 093 | 0 | 8 | 7 | 8 | 0 | ✅ (Completed) | N/A | [093_fix_from_static_routine_transfer](./completed/093_fix_from_static_routine_transfer.md) | Fix From<StaticCommandRegistry> to transfer routines; fix Pipeline::from_static() doctest |
| 9 | 084 | 0 | 8 | 7 | 8 | 0 | ✅ (Completed) | N/A | [084_help_formatting_improvements](./completed/084_help_formatting_improvements.md) | Already implemented - All features exist: categorization, prefix filtering, hidden commands |
| 10 | 085 | 0 | 10 | 4 | 5 | 0 | ✅ (Completed) | N/A | [085_make_illegal_states_unrepresentable](./completed/085_make_illegal_states_unrepresentable.md) | Build-time validation prevents illegal states - Task 085 resolved (8/10 items) |
| 11 | 086 | 0 | 9 | 6 | 7 | 0 | ✅ (Completed) | N/A | [086_prevent_argv_misuse_pitfall](./completed/086_prevent_argv_misuse_pitfall.md) | Prevent argv→string→split misuse through API redesign and documentation |
| 12 | 087 | 0 | 9 | 6 | 6 | 0 | ✅ (Completed) | N/A | [087_prevent_command_help_divergence](./completed/087_prevent_command_help_divergence.md) | Make command/help divergence impossible through registry API enforcement - Phase 1 complete: auto-help generation, format_command_listing(), validate_help_completeness() |
| 13 | 088 | 0 | 9 | 7 | 6 | 0 | ✅ (Completed) | N/A | [088_fix_auto_help_enabled_conversion_bug](./completed/088_fix_auto_help_enabled_conversion_bug.md) | Fix auto_help_enabled lost during Static-to-Dynamic conversion |
| 14 | 089 | 0 | 8 | 7 | 8 | 0 | ✅ (Completed) | N/A | [089_extract_output_truncation](./completed/089_extract_output_truncation.md) | Extract output truncation utilities (head/tail/width) with ANSI/Unicode support |
| 15 | 090 | 0 | 6 | 8 | 8 | 0 | ✅ (Completed) | N/A | [090_extract_config_extraction_functions](./completed/090_extract_config_extraction_functions.md) | Extract config value extraction functions for CliParamsAdvanced ecosystem |
| 16 | 091 | 0 | 5 | 7 | 8 | 0 | ✅ (Completed) | N/A | [091_extract_verbosity_logging](./completed/091_extract_verbosity_logging.md) | Extract verbosity-based logging - REJECTED: Use tracing crate instead |
| 17 | 083 | 0 | 6 | 4 | 5 | 0 | ✅ (Completed) | N/A | [083_implement_preserved_quotes_stripping](./completed/083_implement_preserved_quotes_stripping.md) | Obsolete - Solved via issue-084 with different approach (preserve quotes, don't strip) |
| 18 | 078 | 0 | 9 | 8 | 5 | 0 | ✅ (Completed) | N/A | [078_update_cargo_dependencies](./completed/078_update_cargo_dependencies.md) | Update Cargo dependencies for new functionality |
| 19 | 082 | 0 | 9 | 9 | 7 | 0 | ✅ (Completed) | N/A | [082_fix_whitespace_detection_bug](./completed/082_fix_whitespace_detection_bug.md) | Fix whitespace detection bug in parse_from_argv |
| 20 | 056 | 0 | 9 | 6 | 5 | 0 | ✅ (Completed) | N/A | [056_write_tests_for_static_data_structures_extension](./completed/056_write_tests_for_static_data_structures_extension.md) | Write tests for static data structures extension |
| 21 | 058 | 0 | 9 | 6 | 5 | 0 | ✅ (Completed) | N/A | [058_write_tests_for_phf_map_generation_system](./completed/058_write_tests_for_phf_map_generation_system.md) | Write tests for PHF map generation system |
| 22 | 060 | 0 | 9 | 6 | 5 | 0 | ✅ (Completed) | N/A | [060_write_tests_for_static_command_registry](./completed/060_write_tests_for_static_command_registry.md) | Write tests for StaticCommandRegistry |
| 23 | 062 | 0 | 9 | 6 | 5 | 0 | ✅ (Completed) | N/A | [062_write_tests_for_registry_integration](./completed/062_write_tests_for_registry_integration.md) | Write tests for registry integration |
| 24 | 065 | 0 | 8 | 6 | 5 | 0 | ✅ (Completed) | N/A | [065_write_tests_for_cli_builder_api](./completed/065_write_tests_for_cli_builder_api.md) | Write tests for CliBuilder API |
| 25 | 067 | 0 | 8 | 6 | 5 | 0 | ✅ (Completed) | N/A | [067_write_tests_for_multi_yaml_system](./completed/067_write_tests_for_multi_yaml_system.md) | Write tests for multi-YAML system |
| 26 | 061 | 0 | 9 | 4 | 5 | 0 | ✅ (Completed) | N/A | [061_implement_static_command_registry](./completed/061_implement_static_command_registry.md) | Implement StaticCommandRegistry |
| 27 | 063 | 0 | 9 | 4 | 5 | 0 | ✅ (Completed) | N/A | [063_implement_registry_integration](./completed/063_implement_registry_integration.md) | Implement registry integration |
| 28 | 057 | 0 | 9 | 4 | 5 | 0 | ✅ (Completed) | N/A | [057_implement_static_data_structures_extension](./completed/057_implement_static_data_structures_extension.md) | Implement static data structures extension |
| 29 | 059 | 0 | 9 | 4 | 5 | 0 | ✅ (Completed) | N/A | [059_implement_phf_map_generation_system](./completed/059_implement_phf_map_generation_system.md) | Implement PHF map generation system |
| 30 | 081 | 0 | 9 | 8 | 5 | 0 | ✅ (Completed) | N/A | [081_write_tests_for_whitespace_detection_bug](./completed/081_write_tests_for_whitespace_detection_bug.md) | Write tests for whitespace detection bug in parse_from_argv |
| 31 | 048 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | N/A | [048_write_tests_for_hybrid_registry_optimization](./completed/048_write_tests_for_hybrid_registry_optimization.md) | Write tests for hybrid registry optimization |
| 32 | 049 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | N/A | [049_implement_hybrid_registry_optimization](./completed/049_implement_hybrid_registry_optimization.md) | Implement hybrid registry optimization |
| 33 | 050 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | N/A | [050_write_tests_for_multi_yaml_build_system](./completed/050_write_tests_for_multi_yaml_build_system.md) | Write tests for multi-YAML build system |
| 34 | 051 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | N/A | [051_implement_multi_yaml_build_system](./completed/051_implement_multi_yaml_build_system.md) | Implement multi-YAML build system |
| 35 | 052 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | N/A | [052_write_tests_for_ergonomic_aggregation_apis](./completed/052_write_tests_for_ergonomic_aggregation_apis.md) | Write tests for ergonomic aggregation APIs |
| 36 | 053 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | N/A | [053_implement_ergonomic_aggregation_apis](./completed/053_implement_ergonomic_aggregation_apis.md) | Implement ergonomic aggregation APIs |
| 37 | 054 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | N/A | [054_write_tests_for_performance_optimizations](./completed/054_write_tests_for_performance_optimizations.md) | Write tests for performance optimizations |
| 38 | 055 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | N/A | [055_implement_performance_optimizations](./completed/055_implement_performance_optimizations.md) | Implement performance optimizations |
| 39 | 066 | 0 | 8 | 4 | 5 | 0 | ✅ (Completed) | N/A | [066_implement_cli_builder_api](./completed/066_implement_cli_builder_api.md) | Implement CliBuilder API |
| 40 | 068 | 0 | 8 | 4 | 5 | 0 | ✅ (Completed) | N/A | [068_implement_multi_yaml_system](./completed/068_implement_multi_yaml_system.md) | Implement multi-YAML system |
| 41 | 064 | 0 | 10 | 6 | 5 | 0 | ✅ (Completed) | N/A | [064_enable_static_command_examples](./completed/064_enable_static_command_examples.md) | Enable static command examples |
| 42 | 069 | 0 | 10 | 6 | 5 | 0 | ✅ (Completed) | N/A | [069_enable_cli_aggregation_examples](./completed/069_enable_cli_aggregation_examples.md) | Enable CLI aggregation examples |
| 43 | 044 | 0 | 7 | 8 | 9 | 0 | ✅ (Completed) | N/A | [044_fix_documentation_warnings_and_debug_implementations](./completed/044_fix_documentation_warnings_and_debug_implementations.md) | Fix documentation warnings and missing Debug implementations |
| 44 | 042 | 0 | 6 | 6 | 7 | 0 | ✅ (Completed) | N/A | [042_add_context_rich_benchmark_documentation](./completed/042_add_context_rich_benchmark_documentation.md) | Add context-rich benchmark documentation |
| 45 | 043 | 0 | 6 | 6 | 7 | 0 | ✅ (Completed) | N/A | [043_implement_before_after_optimization_workflow](./completed/043_implement_before_after_optimization_workflow.md) | Implement before/after optimization workflow |
| 46 | 045 | 0 | 6 | 9 | 9 | 0 | ✅ (Completed) | N/A | [045_move_completed_tasks_to_completed_directory](./completed/045_move_completed_tasks_to_completed_directory.md) | Move completed tasks to completed directory |
| 47 | 070 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | N/A | [070_write_tests_for_documentation_updater](./completed/070_write_tests_for_documentation_updater.md) | Write tests for documentation updater |
| 48 | 071 | 0 | 8 | 4 | 7 | 0 | ✅ (Completed) | N/A | [071_implement_documentation_updater](./completed/071_implement_documentation_updater.md) | Implement documentation updater |
| 49 | 072 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | N/A | [072_write_tests_for_benchmark_configuration_system](./completed/072_write_tests_for_benchmark_configuration_system.md) | Write tests for benchmark configuration system |
| 50 | 074 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | N/A | [074_write_tests_for_performance_analysis_tools](./completed/074_write_tests_for_performance_analysis_tools.md) | Write tests for performance analysis tools |
| 51 | 077 | 0 | 10 | 4 | 5 | 0 | ✅ (Completed) | N/A | [077_final_integration_testing](./completed/077_final_integration_testing.md) | Final integration testing |
| 52 | 047 | 0 | 8 | 6 | 8 | 0 | ✅ (Completed) | N/A | [047_verify_benchmark_execution_functionality](./completed/047_verify_benchmark_execution_functionality.md) | Verify benchmark execution functionality |
| 53 | 046 | 0 | 4 | 10 | 9 | 0 | ✅ (Completed) | N/A | [046_remove_obsolete_task_artifacts](./completed/046_remove_obsolete_task_artifacts.md) | Remove obsolete task artifacts |
| 54 | 026 | 0 | 8 | 9 | 4 | 0 | ✅ (Completed) | N/A | [026_remove_obsolete_throughput_benchmark_original](./completed/026_remove_obsolete_throughput_benchmark_original.md) | Remove obsolete throughput benchmark original |
| 55 | 033 | 0 | 8 | 7 | 5 | 0 | ✅ (Completed) | N/A | [033_fix_generic_section_naming_violations](./completed/033_fix_generic_section_naming_violations.md) | Fix generic section naming violations |
| 56 | 034 | 0 | 8 | 7 | 5 | 0 | ✅ (Completed) | N/A | [034_replace_custom_scripts_with_cargo_bench](./completed/034_replace_custom_scripts_with_cargo_bench.md) | Replace custom scripts with cargo bench workflow |
| 57 | 035 | 0 | 8 | 7 | 5 | 0 | ✅ (Completed) | N/A | [035_implement_statistical_significance_testing](./completed/035_implement_statistical_significance_testing.md) | Implement statistical significance testing |
| 58 | 036 | 0 | 8 | 7 | 5 | 0 | ✅ (Completed) | N/A | [036_implement_environment_specific_cv_configuration](./completed/036_implement_environment_specific_cv_configuration.md) | Implement environment-specific CV configuration |
| 59 | 028 | 0 | 9 | 7 | 4 | 0 | ✅ (Completed) | N/A | [028_fix_benchmarks_directory_structure](./completed/028_fix_benchmarks_directory_structure.md) | Fix benchmarks directory structure |
| 60 | 029 | 0 | 9 | 7 | 4 | 0 | ✅ (Completed) | N/A | [029_implement_benchkit_standard_setup_protocol](./completed/029_implement_benchkit_standard_setup_protocol.md) | Implement benchkit standard setup protocol |
| 61 | 030 | 0 | 9 | 7 | 4 | 0 | ✅ (Completed) | N/A | [030_implement_coefficient_of_variation_analysis](./completed/030_implement_coefficient_of_variation_analysis.md) | Implement coefficient of variation analysis |
| 62 | 031 | 0 | 9 | 7 | 4 | 0 | ✅ (Completed) | N/A | [031_add_measurement_context_templates](./completed/031_add_measurement_context_templates.md) | Add measurement context templates |
| 63 | 032 | 0 | 9 | 7 | 4 | 0 | ✅ (Completed) | N/A | [032_implement_automatic_documentation_updates](./completed/032_implement_automatic_documentation_updates.md) | Implement automatic documentation updates |
| 64 | 039 | 0 | 6 | 6 | 7 | 0 | ✅ (Completed) | N/A | [039_standardize_benchmark_data_sizes](./completed/039_standardize_benchmark_data_sizes.md) | Standardize benchmark data sizes |
| 65 | 040 | 0 | 6 | 6 | 7 | 0 | ✅ (Completed) | N/A | [040_implement_realistic_test_data_generation](./completed/040_implement_realistic_test_data_generation.md) | Implement realistic test data generation |
| 66 | 041 | 0 | 6 | 6 | 7 | 0 | ✅ (Completed) | N/A | [041_implement_comparative_benchmark_structure](./completed/041_implement_comparative_benchmark_structure.md) | Implement comparative benchmark structure |
| 67 | 001 | 0 | 5 | 5 | 8 | 0 | ✅ (Completed) | N/A | [001_string_interning_system](./completed/001_string_interning_system.md) | String interning system implementation |
| 68 | 003 | 0 | 8 | 5 | 5 | 0 | ✅ (Completed) | N/A | [003_phase3](./completed/003_phase3.md) | Phase 3 implementation |
| 69 | 004 | 0 | 5 | 5 | 8 | 0 | ✅ (Completed) | N/A | [004_simd_tokenization](./completed/004_simd_tokenization.md) | SIMD tokenization implementation |
| 70 | 005 | 0 | 8 | 5 | 5 | 0 | ✅ (Completed) | N/A | [005_phase4](./completed/005_phase4.md) | Phase 4 implementation |
| 71 | 006 | 0 | 8 | 5 | 5 | 0 | ✅ (Completed) | N/A | [006_phase3_finalize](./completed/006_phase3_finalize.md) | Phase 3 completion tasks |
| 72 | 009 | 0 | 8 | 5 | 5 | 0 | ✅ (Completed) | N/A | [009_simd_json_parsing](./completed/009_simd_json_parsing.md) | SIMD JSON parsing implementation |
| 73 | 011 | 0 | 8 | 5 | 5 | 0 | ✅ (Completed) | N/A | [011_strs_tools_simd_ref](./completed/011_strs_tools_simd_ref.md) | Strs tools SIMD reference implementation |
| 74 | 013 | 0 | 8 | 5 | 5 | 0 | ✅ (Completed) | N/A | [013_phase5](./completed/013_phase5.md) | Phase 5 implementation |
| 75 | 014 | 0 | 5 | 5 | 8 | 0 | ✅ (Completed) | N/A | [014_wasm](./completed/014_wasm.md) | WebAssembly support implementation |
| 76 | 016 | 0 | 5 | 5 | 8 | 0 | ✅ (Completed) | N/A | [016_phase6](./completed/016_phase6.md) | Phase 6 implementation |
| 77 | 017 | 0 | 8 | 5 | 5 | 0 | ✅ (Completed) | N/A | [017_issue_command_runtime_registration_failure](./completed/017_issue_command_runtime_registration_failure.md) | Fix command runtime registration failure |
| 78 | 018 | 0 | 8 | 5 | 5 | 0 | ✅ (Completed) | N/A | [018_documentation_enhanced_repl_features](./completed/018_documentation_enhanced_repl_features.md) | Enhanced REPL features documentation |
| 79 | 020 | 0 | 8 | 5 | 4 | 0 | ✅ (Completed) | N/A | [020_fix_throughput_benchmark_api](./completed/020_fix_throughput_benchmark_api.md) | Fix API mismatches in benchmarks/throughput_benchmark.rs |
| 80 | 019 | 0 | 7 | 5 | 4 | 0 | ✅ (Completed) | N/A | [019_api_consistency_command_result](./completed/019_api_consistency_command_result.md) | API consistency for command results |
| 81 | 021 | 0 | 7 | 5 | 4 | 0 | ✅ (Completed) | N/A | [021_modernize_simple_json_perf_test](./completed/021_modernize_simple_json_perf_test.md) | Convert simple_json_perf_test.rs to use benchkit properly |
| 82 | 022 | 0 | 7 | 5 | 4 | 0 | ✅ (Completed) | N/A | [022_fix_simd_performance_validation](./completed/022_fix_simd_performance_validation.md) | Update SIMD performance validation test to use benchkit |
| 83 | 023 | 0 | 7 | 5 | 4 | 0 | ✅ (Completed) | N/A | [023_modernize_performance_stress_test](./completed/023_modernize_performance_stress_test.md) | Convert performance stress test to benchkit compliance |
| 84 | 027 | 0 | 3 | 10 | 4 | 0 | ✅ (Completed) | N/A | [027_update_benchkit_integration_demo_ignore_message](./completed/027_update_benchkit_integration_demo_ignore_message.md) | Update benchkit integration demo ignore message |
| 85 | 002 | 0 | 5 | 5 | 4 | 0 | ✅ (Completed) | N/A | [002_zero_copy_parser_tokens_ref](./completed/002_zero_copy_parser_tokens_ref.md) | Zero-copy parser tokens optimization |
| 86 | 024 | 0 | 6 | 4 | 4 | 0 | ✅ (Completed) | N/A | [024_convert_comprehensive_framework_comparison_to_benchkit](./completed/024_convert_comprehensive_framework_comparison_to_benchkit.md) | Convert comprehensive framework comparison to benchkit |
| 87 | 079 | 0 | 9 | 2 | 5 | 0 | ✅ (Completed) | N/A | [079_fix_multiple_parameter_handling](./completed/079_fix_multiple_parameter_handling.md) | Fix multiple parameter handling |
| 88 | 025 | 0 | 5 | 3 | 4 | 0 | ✅ (Completed) | N/A | [025_convert_run_all_benchmarks_to_benchkit](./completed/025_convert_run_all_benchmarks_to_benchkit.md) | Convert run all benchmarks suite to benchkit |
| 89 | 080 | 0 | 10 | 1 | 5 | 0 | ✅ (Completed) | N/A | [080_argv_based_api_request](./completed/080_argv_based_api_request.md) | Add argv-based API to unilang for proper CLI integration |
| 90 | 099 | 0 | 7 | 6 | 9 | 0 | ✅ (Completed) | N/A | [099_solution_shell_argument_handling](./completed/099_solution_shell_argument_handling.md) | [doc] Shell argument handling investigation and solution notes |
| 91 | 100 | 0 | 6 | 7 | 8 | 0 | ✅ (Completed) | N/A | [100_help_formatter_hide_empty_fields](./completed/100_help_formatter_hide_empty_fields.md) | [doc] Help formatter issue report and acceptance criteria |
| 92 | 101 | 0 | 8 | 5 | 7 | 0 | ✅ (Completed) | N/A | [101_phf_dependency_elimination](./completed/101_phf_dependency_elimination.md) | [doc] PHF dependency elimination solution summary |
| 0 | 105 | 0 | 7 | 7 | 9 | 0 | ✅ (Completed) | claude-sonnet-4-6 | [105_implement_build_runtime_separation_tests](./completed/105_implement_build_runtime_separation_tests.md) | Implement 4 test cases for invariant/006 build-runtime separation spec |

## Issues Index

| ID | Title | Related Task | Status |
|----|-------|--------------|--------|
| BUG-102 | [.help command visible in its own help listing](./bug/closed/102_help_self_referential_visibility.md) | 103 | Fixed |

