-- Port Allocation for CPP Repositories
--
-- This is the single source of truth for port assignments.
-- All repos import this to ensure no conflicts.
--
-- Authors: Fourth Transition Initiative
-- Operator: Cool Planet Foundation

{
  fti_newsletter_archive = {
    dfx = 8000,
    vite = 3000
  },

  cpf_members = {
    dfx = 8001,
    vite = 3002
  },

  cpf_org = {
    dfx = 8002,
    vite = 3001
  },

  cpp_icp_platform = {
    dfx = 8003
    -- No vite (SDK library repo, no frontend dev server)
  }
}
