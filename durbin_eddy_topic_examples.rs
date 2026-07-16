//! Scaffold file for building Rust examples from the non-intro chapters of
//! Durbin et al., *Biological Sequence Analysis*.
//!
//! Each function is intentionally empty so you can add an example implementation
//! for the corresponding topic.

#[allow(dead_code)]
pub mod pairwise_alignment {
    pub fn introduction() {}
    pub fn the_scoring_model() {}
    pub fn alignment_algorithms() {}
    pub fn dynamic_programming_with_more_complex_models() {}
    pub fn heuristic_alignment_algorithms() {}
    pub fn linear_space_alignments() {}
    pub fn significance_of_scores() {}
    pub fn deriving_score_parameters_from_alignment_data() {}
    pub fn further_reading() {}
}

#[allow(dead_code)]
pub mod markov_chains_and_hidden_markov_models {
    pub fn markov_chains() {}
    pub fn hidden_markov_models() {}
    pub fn parameter_estimation_for_hmms() {}
    pub fn hmm_model_structure() {}
    pub fn more_complex_markov_chains() {}
    pub fn numerical_stability_of_hmm_algorithms() {}
    pub fn further_reading() {}
}

#[allow(dead_code)]
pub mod pairwise_alignment_using_hmms {
    pub fn pair_hmms() {}
    pub fn full_probability_of_x_and_y_summing_over_all_paths() {}
    pub fn suboptimal_alignment() {}
    pub fn posterior_probability_that_xi_is_aligned_to_yj() {}
    pub fn pair_hmms_versus_fsas_for_searching() {}
    pub fn further_reading() {}
}

#[allow(dead_code)]
pub mod profile_hmms_for_sequence_families {
    pub fn ungapped_score_matrices() {}
    pub fn adding_insert_and_delete_states_to_obtain_profile_hmms() {}
    pub fn deriving_profile_hmms_from_multiple_alignments() {}
    pub fn searching_with_profile_hmms() {}
    pub fn profile_hmm_variants_for_non_global_alignments() {}
    pub fn more_on_estimation_of_probabilities() {}
    pub fn optimal_model_construction() {}
    pub fn weighting_training_sequences() {}
    pub fn further_reading() {}
}

#[allow(dead_code)]
pub mod multiple_sequence_alignment_methods {
    pub fn what_a_multiple_alignment_means() {}
    pub fn scoring_a_multiple_alignment() {}
    pub fn multidimensional_dynamic_programming() {}
    pub fn progressive_alignment_methods() {}
    pub fn multiple_alignment_by_profile_hmm_training() {}
    pub fn further_reading() {}
}

#[allow(dead_code)]
pub mod building_phylogenetic_trees {
    pub fn the_tree_of_life() {}
    pub fn background_on_trees() {}
    pub fn making_a_tree_from_pairwise_distances() {}
    pub fn parsimony() {}
    pub fn assessing_the_trees_the_bootstrap() {}
    pub fn simultaneous_alignment_and_phylogeny() {}
    pub fn further_reading() {}
}

#[allow(dead_code)]
pub mod appendix_proof_of_neighbour_joining_theorem {
    pub fn proof_of_neighbour_joining_theorem() {}
}

#[allow(dead_code)]
pub mod probabilistic_approaches_to_phylogeny {
    pub fn introduction() {}
    pub fn probabilistic_models_of_evolution() {}
    pub fn calculating_the_likelihood_for_ungapped_alignments() {}
    pub fn using_the_likelihood_for_inference() {}
    pub fn towards_more_realistic_evolutionary_models() {}
    pub fn comparison_of_probabilistic_and_non_probabilistic_methods() {}
    pub fn further_reading() {}
}

#[allow(dead_code)]
pub mod transformational_grammars {
    pub fn transformational_grammars_topic() {}
    pub fn regular_grammars() {}
    pub fn context_free_grammars() {}
    pub fn context_sensitive_grammars() {}
    pub fn stochastic_grammars() {}
    pub fn stochastic_context_free_grammars_for_sequence_modelling() {}
    pub fn further_reading() {}
}

#[allow(dead_code)]
pub mod rna_structure_analysis {
    pub fn rna() {}
    pub fn rna_secondary_structure_prediction() {}
    pub fn covariance_models_scfg_based_rna_profiles() {}
    pub fn further_reading() {}
}

#[allow(dead_code)]
pub mod background_on_probability {
    pub fn probability_distributions() {}
    pub fn entropy() {}
    pub fn inference() {}
    pub fn sampling() {}
    pub fn estimation_of_probabilities_from_counts() {}
    pub fn the_em_algorithm() {}
}
