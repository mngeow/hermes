use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use serde_yaml::Value;

use crate::models::AgentMode;

#[derive(Debug, Clone)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct AgentFrontmatter {
    pub description: String,
    pub mode: Option<AgentMode>,
    pub has_external_prompt: bool,
    pub has_body: bool,
}

#[derive(Debug, Deserialize)]
struct RawSkillFrontmatter {
    name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct RawAgentFrontmatter {
    description: Option<String>,
    mode: Option<String>,
    prompt: Option<Value>,
}

pub fn parse_skill_file(path: &Path) -> Result<SkillFrontmatter> {
    let (yaml, _) = read_frontmatter(path)?;
    let raw: RawSkillFrontmatter = serde_yaml::from_str(&yaml)
        .with_context(|| format!("failed to parse skill frontmatter in {}", path.display()))?;

    if raw.name.trim().is_empty() {
        bail!("skill frontmatter name must not be empty")
    }
    if raw.description.trim().is_empty() {
        bail!("skill frontmatter description must not be empty")
    }

    Ok(SkillFrontmatter {
        name: raw.name,
        description: raw.description,
    })
}

pub fn parse_agent_file(path: &Path) -> Result<AgentFrontmatter> {
    let (yaml, body) = read_frontmatter(path)?;
    let raw: RawAgentFrontmatter = serde_yaml::from_str(&yaml)
        .with_context(|| format!("failed to parse agent frontmatter in {}", path.display()))?;

    let description = raw
        .description
        .ok_or_else(|| anyhow::anyhow!("agent frontmatter is missing required description"))?;
    if description.trim().is_empty() {
        bail!("agent frontmatter description must not be empty")
    }

    let mode = raw.mode.as_deref().map(AgentMode::parse).transpose()?;
    let has_external_prompt = matches!(
        raw.prompt,
        Some(Value::Mapping(map)) if map.contains_key(Value::String("file".to_string()))
    );

    Ok(AgentFrontmatter {
        description,
        mode,
        has_external_prompt,
        has_body: !body.trim().is_empty(),
    })
}

fn read_frontmatter(path: &Path) -> Result<(String, String)> {
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    split_frontmatter(&content)
        .with_context(|| format!("invalid frontmatter in {}", path.display()))
}

fn split_frontmatter(content: &str) -> Result<(String, String)> {
    let mut lines = content.lines();
    match lines.next() {
        Some(line) if line.trim() == "---" => {}
        _ => bail!("missing opening frontmatter delimiter"),
    }

    let mut yaml_lines = Vec::new();
    let mut found_end = false;
    for line in lines.by_ref() {
        if line.trim() == "---" {
            found_end = true;
            break;
        }
        yaml_lines.push(line);
    }

    if !found_end {
        bail!("missing closing frontmatter delimiter")
    }

    let body = lines.collect::<Vec<_>>().join("\n");
    Ok((yaml_lines.join("\n"), body))
}
