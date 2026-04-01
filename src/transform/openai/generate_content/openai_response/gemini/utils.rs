use crate::gemini::generate_content::types as gt;
use crate::openai::count_tokens::types as ot;
use crate::openai::create_response::types as rt;

pub fn gemini_citation_annotations(
    citation_metadata: Option<&gt::GeminiCitationMetadata>,
) -> Vec<ot::ResponseOutputTextAnnotation> {
    citation_metadata
        .and_then(|metadata| metadata.citation_sources.as_ref())
        .map(|sources| {
            sources
                .iter()
                .filter_map(|source| {
                    let url = source.uri.clone()?;
                    Some(ot::ResponseOutputTextAnnotation::UrlCitation(
                        ot::ResponseUrlCitation {
                            start_index: source.start_index.unwrap_or(0) as u64,
                            end_index: source.end_index.unwrap_or(0) as u64,
                            title: url.clone(),
                            type_: ot::ResponseUrlCitationType::UrlCitation,
                            url,
                        },
                    ))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub fn gemini_logprobs(
    result: Option<&gt::GeminiLogprobsResult>,
) -> Option<Vec<ot::ResponseOutputTokenLogprob>> {
    let result = result?;
    let chosen_candidates = result.chosen_candidates.as_ref()?;
    let top_candidates = result.top_candidates.as_ref();

    let mut output = Vec::new();
    for (index, chosen) in chosen_candidates.iter().enumerate() {
        let token = chosen.token.clone().unwrap_or_default();
        if token.is_empty() {
            continue;
        }

        let top_logprobs = top_candidates
            .and_then(|list| list.get(index))
            .and_then(|item| item.candidates.as_ref())
            .map(|candidates| {
                candidates
                    .iter()
                    .filter_map(|candidate| {
                        let token = candidate.token.clone()?;
                        Some(ot::ResponseOutputTopLogprob {
                            token,
                            bytes: None,
                            logprob: candidate.log_probability.unwrap_or_default(),
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        output.push(ot::ResponseOutputTokenLogprob {
            token,
            bytes: None,
            logprob: chosen.log_probability.unwrap_or_default(),
            top_logprobs,
        });
    }

    if output.is_empty() {
        None
    } else {
        Some(output)
    }
}

pub fn grounding_sources(
    grounding_metadata: Option<&gt::GeminiGroundingMetadata>,
) -> Option<Vec<ot::ResponseFunctionWebSearchSource>> {
    let chunks = grounding_metadata.and_then(|metadata| metadata.grounding_chunks.as_ref())?;
    let mut sources = Vec::new();

    for chunk in chunks {
        if let Some(web) = chunk.web.as_ref() {
            sources.push(ot::ResponseFunctionWebSearchSource {
                type_: ot::ResponseFunctionWebSearchSourceType::Url,
                url: web.uri.clone(),
            });
        }
        if let Some(retrieved) = chunk.retrieved_context.as_ref()
            && let Some(uri) = retrieved.uri.as_ref()
        {
            sources.push(ot::ResponseFunctionWebSearchSource {
                type_: ot::ResponseFunctionWebSearchSourceType::Url,
                url: uri.clone(),
            });
        }
        if let Some(maps) = chunk.maps.as_ref()
            && let Some(uri) = maps.uri.as_ref()
        {
            sources.push(ot::ResponseFunctionWebSearchSource {
                type_: ot::ResponseFunctionWebSearchSourceType::Url,
                url: uri.clone(),
            });
        }
    }

    if sources.is_empty() {
        None
    } else {
        Some(sources)
    }
}

pub fn gemini_grounding_to_web_search_item(
    candidate_index: u32,
    grounding_metadata: Option<&gt::GeminiGroundingMetadata>,
) -> Option<rt::ResponseOutputItem> {
    let grounding_metadata = grounding_metadata?;
    let queries = grounding_metadata
        .web_search_queries
        .clone()
        .unwrap_or_default();
    let sources = grounding_sources(Some(grounding_metadata));

    if queries.is_empty() && sources.is_none() {
        return None;
    }

    Some(rt::ResponseOutputItem::FunctionWebSearch(
        ot::ResponseFunctionWebSearch {
            id: Some(format!("web_search_{candidate_index}")),
            action: ot::ResponseFunctionWebSearchAction::Search {
                query: queries.first().cloned(),
                queries: if queries.len() > 1 {
                    Some(queries)
                } else {
                    None
                },
                sources,
            },
            status: ot::ResponseFunctionWebSearchStatus::Completed,
            type_: ot::ResponseFunctionWebSearchType::WebSearchCall,
        },
    ))
}
