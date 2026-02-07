# This is a Ruby comment
class Person
  attr_accessor :name, :age
  
  def initialize(name, age)
    @name = name
    @age = age
  end
end

def greet(name)
  message = "Hello, #{name}"
  message
end

user_name = "World"
puts greet(user_name)
