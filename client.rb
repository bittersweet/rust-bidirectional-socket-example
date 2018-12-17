require "socket"
require "thread"

def doit(input)
  UNIXSocket.open("/tmp/rust_socket.tmp") do |socket|
    # Note: newline is important here
    socket.write(input.to_s + "\n")

    while(line = socket.gets) do
      puts line
    end
  end
end

jobs = []
10.times do |i|
  jobs << Thread.new do
    doit("Ping #{i}")
  end
end

jobs.map(&:join)
