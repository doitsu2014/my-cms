// Claude Code payload metadata. The source tree is copied into
// .claude/skills/dtech-aidlc and the host discovers SKILL.md as /dtech-aidlc.
const manifest = {
  name: "claude",
  productName: "Claude Code",
  harnessDir: ".claude",
  invoke: "/dtech-aidlc",
  configNextStep: "Open Claude Code in this project and run /dtech-aidlc --doctor.",
  orchestratorSkillPath: ".claude/skills/dtech-aidlc/SKILL.md",
};

export default manifest;
