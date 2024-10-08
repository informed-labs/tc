
Gem::Specification.new do |spec|
  spec.name          = 'bar'
  spec.version       = '1.0'
  spec.authors       = ['icy']
  spec.email         = ['icy@spicy.com']

  spec.required_ruby_version = '>= 3.2'

  spec.summary       = 'bar lib'
  spec.description   = 'Provides functionality'
  spec.homepage      = 'https://www.tc.com/'
  spec.license       = 'whatever.'

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(File.expand_path(__dir__)) do
    `git ls-files -z`.split("\x0").reject do |f|
      (f == __FILE__) || f.match(%r{\A(?:(?:test|spec|features)/|\.(?:git|travis|circleci)|appveyor)})
    end
  end
  spec.require_paths = ['lib']

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"
end
