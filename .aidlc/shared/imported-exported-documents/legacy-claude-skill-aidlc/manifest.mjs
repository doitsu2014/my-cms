// Claude Code payload metadata. The source tree is copied into
// .claude/skills/aidlc and the host discovers SKILL.md as /aidlc.
const manifest = {
  name: "claude",
  productName: "Claude Code",
  harnessDir: ".claude",
  invoke: "/aidlc",
  configNextStep: "Open Claude Code in this project and run /aidlc --doctor.",
  orchestratorSkillPath: ".claude/skills/aidlc/SKILL.md",
};

export default manifest;
