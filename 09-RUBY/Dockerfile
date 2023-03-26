# Base image
FROM ruby:2.7

# Set working directory
WORKDIR /app

# Copy application files
COPY . /app

# Install dependencies
RUN bundle install

# Run the application
CMD ["ruby", "app.rb"]
