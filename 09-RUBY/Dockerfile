# Base image
FROM ruby:2.7

# Set working directory
WORKDIR /app

# Copy application files
COPY app.rb Gemfile /app

# Install dependencies
RUN bundle install

#set timezone tehran
RUN  rm -f /etc/localtime && ln -s /usr/share/zoneinfo/Asia/Tehran  /etc/localtime

# Run the application
CMD ["ruby", "app.rb"]
