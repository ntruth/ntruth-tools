import { Component } from 'solid-js'
import { Heart, Github, Globe, Mail, Coffee, Star } from 'lucide-solid'

const About: Component = () => {
  const openUrl = (url: string) => {
    window.open(url, '_blank')
  }

  return (
    <div class="space-y-6">
      <div class="text-center">
        {/* Logo */}
        <div class="flex justify-center mb-4">
          <div class="flex h-20 w-20 items-center justify-center rounded-2xl bg-gradient-to-br from-blue-500 to-purple-600 text-white shadow-lg">
            <span class="text-3xl font-bold">O</span>
          </div>
        </div>
        
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
          OmniBox
        </h1>
        <p class="text-gray-500 dark:text-gray-400">
          Version 0.1.0
        </p>
        <p class="mt-2 text-sm text-gray-600 dark:text-gray-300">
          A powerful cross-platform productivity launcher
        </p>
      </div>

      {/* Features */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="text-sm font-medium text-gray-900 dark:text-white mb-3">
          Features
        </h3>
        <ul class="space-y-2 text-sm text-gray-600 dark:text-gray-400">
          <li class="flex items-center gap-2">
            <Star size={14} class="text-yellow-500" />
            Fast file and application search
          </li>
          <li class="flex items-center gap-2">
            <Star size={14} class="text-yellow-500" />
            Clipboard history management
          </li>
          <li class="flex items-center gap-2">
            <Star size={14} class="text-yellow-500" />
            AI-powered chat assistant
          </li>
          <li class="flex items-center gap-2">
            <Star size={14} class="text-yellow-500" />
            Built-in calculator
          </li>
          <li class="flex items-center gap-2">
            <Star size={14} class="text-yellow-500" />
            Web search integration
          </li>
          <li class="flex items-center gap-2">
            <Star size={14} class="text-yellow-500" />
            Screenshot tools (coming soon)
          </li>
        </ul>
      </div>

      {/* Links */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="text-sm font-medium text-gray-900 dark:text-white mb-3">
          Links
        </h3>
        <div class="space-y-2">
          <button
            onClick={() => openUrl('https://github.com/ntruth/omnibox')}
            class="flex w-full items-center gap-3 rounded-lg p-2 text-left hover:bg-gray-50 dark:hover:bg-gray-800"
          >
            <Github size={18} class="text-gray-600 dark:text-gray-400" />
            <span class="text-sm text-gray-700 dark:text-gray-300">
              GitHub Repository
            </span>
          </button>
          <button
            onClick={() => openUrl('https://omnibox.dev')}
            class="flex w-full items-center gap-3 rounded-lg p-2 text-left hover:bg-gray-50 dark:hover:bg-gray-800"
          >
            <Globe size={18} class="text-gray-600 dark:text-gray-400" />
            <span class="text-sm text-gray-700 dark:text-gray-300">
              Website
            </span>
          </button>
          <button
            onClick={() => openUrl('mailto:support@omnibox.dev')}
            class="flex w-full items-center gap-3 rounded-lg p-2 text-left hover:bg-gray-50 dark:hover:bg-gray-800"
          >
            <Mail size={18} class="text-gray-600 dark:text-gray-400" />
            <span class="text-sm text-gray-700 dark:text-gray-300">
              Contact Support
            </span>
          </button>
        </div>
      </div>

      {/* Support */}
      <div class="rounded-lg bg-gradient-to-r from-pink-50 to-purple-50 p-4 dark:from-pink-900/20 dark:to-purple-900/20">
        <div class="flex items-center gap-3">
          <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-pink-100 dark:bg-pink-900">
            <Heart size={20} class="text-pink-600 dark:text-pink-400" />
          </div>
          <div class="flex-1">
            <h3 class="text-sm font-medium text-gray-900 dark:text-white">
              Support Development
            </h3>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              If you find OmniBox useful, consider supporting the project
            </p>
          </div>
          <button
            onClick={() => openUrl('https://buymeacoffee.com/omnibox')}
            class="flex items-center gap-2 rounded-lg bg-yellow-400 px-4 py-2 text-sm font-medium text-yellow-900 hover:bg-yellow-500"
          >
            <Coffee size={16} />
            Donate
          </button>
        </div>
      </div>

      {/* Credits */}
      <div class="text-center text-sm text-gray-500 dark:text-gray-400">
        <p>Built with ❤️ using Tauri, Solid.js, and Rust</p>
        <p class="mt-1">© 2024 ntruth. All rights reserved.</p>
      </div>

      {/* Acknowledgments */}
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <h3 class="text-sm font-medium text-gray-900 dark:text-white mb-3">
          Acknowledgments
        </h3>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          Special thanks to the open-source community and the following projects:
        </p>
        <div class="mt-2 flex flex-wrap gap-2">
          {['Tauri', 'Solid.js', 'Rust', 'TailwindCSS', 'Lucide Icons'].map(
            (tech) => (
              <span class="rounded-full bg-gray-100 px-3 py-1 text-xs text-gray-600 dark:bg-gray-800 dark:text-gray-400">
                {tech}
              </span>
            )
          )}
        </div>
      </div>
    </div>
  )
}

export default About
