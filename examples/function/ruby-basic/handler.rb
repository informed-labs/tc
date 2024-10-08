
module Handler
  class << self
    def handler(event:, context:)
      puts(event)
    end
  end
end
