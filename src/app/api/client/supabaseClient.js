import { createClient } from '@supabase/supabase.js'

const supabaseURL = process.env.VITE_SUPABASE_PUB_URL
const supabaseKey = process.env.VITE_SUPABASE_TEMP_KEY
const supabase = createClient(supabaseURL, supabaseKey)

module.exports = {
  supabaseURL: supabaseURL,
  supabaseKey: supabaseKey,
  supabase: supabase
}