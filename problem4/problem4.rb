class Main
  def run
    arr1 = (100..999).to_a.reverse
    arr2 = (100..999).to_a.reverse

    palindromes = []

    arr1.each do |i|
      arr2.each do |j|
        if is_palindrome?(i*j)
          palindromes << (i*j)
        end
      end
    end

    puts "Palindrome: #{palindromes.max}"
  end

  def is_palindrome?(int)
    str = int.to_s
    str.reverse == str
  end
end

Main.new.run
