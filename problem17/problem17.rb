class Main
  def run
    str = ''
    (1..999).each do |int|
      str += NumberWord.new(int).to_s
    end
    str += 'one thousand'

    str = str.gsub(/ /,'')
    puts str.length
  end
end

class NumberWord
  WORDS = [
    'zero',
    'one',
    'two',
    'three',
    'four',
    'five',
    'six',
    'seven',
    'eight',
    'nine',
    'ten',
    'eleven',
    'twelve',
    'thirteen',
    'fourteen',
    'fifteen',
    'sixteen',
    'seventeen',
    'eighteen',
    'nineteen'
  ]

  WORDS_BY_TENS = [
    '',
    'ten',
    'twenty',
    'thirty',
    'forty',
    'fifty',
    'sixty',
    'seventy',
    'eighty',
    'ninety'
  ]

  def to_s
    str = ''

    if @hundreds_part > 0
      str = WORDS[@hundreds_part] + ' hundred'
      remainder = @int - (@hundreds_part*100)

      if remainder > 0
        return str + ' and ' + NumberWord.new(remainder).to_s
      else
        return str
      end
    else
      if WORDS[@remainder]
        return WORDS[@remainder]
      else
        str = WORDS_BY_TENS[@tens_part]
        if @ones_part > 0
          return str + ' ' + NumberWord.new(@ones_part).to_s
        else
          return str
        end
      end
    end
  end


  def initialize(int)
    @int = int.to_i
    @hundreds_part = @int / 100
    @remainder = (@int - (@hundreds_part*100))
    @tens_part = @remainder / 10
    @ones_part = @remainder - (@tens_part*10)
  end
end


Main.new.run
