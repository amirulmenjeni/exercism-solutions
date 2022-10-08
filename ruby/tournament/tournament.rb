=begin
Write your code for the 'Tournament' exercise in this file. Make the tests in
`tournament_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/tournament` directory.
=end

class Tournament
    def self.write_line(team, scores)
        line = "#{team.ljust(31)}"
        line += "|#{scores[:mp].rjust(3).ljust(4)}"
        line += "|#{scores[:w].rjust(3).ljust(4)}"
        line += "|#{scores[:d].rjust(3).ljust(4)}"
        line += "|#{scores[:l].rjust(3).ljust(4)}"
        line += "|#{scores[:p].rjust(3).ljust(3)}\n"
        line
    end

    def self.get_team_scores(input)
        team_scores = {}

        input.split(/\n/).each do |line|
            s = line.split(';')
            first   = s[0]
            second  = s[1]
            outcome = s[2]

            team_scores[first] ||= {
                :mp => 0,
                :w  => 0,
                :d  => 0,
                :l  => 0,
                :p  => 0
            }

            team_scores[second] ||= {
                :mp => 0,
                :w  => 0,
                :d  => 0,
                :l  => 0,
                :p  => 0
            }

            case [first, second, outcome]
            in [first, second, 'win']
                team_scores[first][:mp] += 1
                team_scores[first][:w]  += 1
                team_scores[first][:p]  += 3

                team_scores[second][:mp] += 1
                team_scores[second][:l]  += 1
                team_scores[second][:p]  += 0

            in [first, second, 'draw']
                team_scores[first][:mp] += 1
                team_scores[first][:d]  += 1
                team_scores[first][:p]  += 1

                team_scores[second][:mp] += 1
                team_scores[second][:d]  += 1
                team_scores[second][:p]  += 1

            in [first, second, 'loss']
                team_scores[first][:mp] += 1
                team_scores[first][:l]  += 1
                team_scores[first][:p]  += 0

                team_scores[second][:mp] += 1
                team_scores[second][:w]  += 1
                team_scores[second][:p]  += 3
            end
        end

        team_scores
    end

    def self.tally(input)
        team_scores = get_team_scores(input)

        # Sort by points first, then by team name.
        sorted_team_scores = team_scores
            .sort
            .sort_by { |team, scores| -scores[:p] }

        output = write_line('Team', {
            :mp => 'MP',
            :w  => 'W',
            :d  => 'D',
            :l  => 'L',
            :p  => 'P'
        })

        sorted_team_scores.each do |team, scores|
            scores.each do |k, v|
                scores[k] = scores[k].to_s
            end
            output += write_line(team, scores)
        end

        return output
    end
end