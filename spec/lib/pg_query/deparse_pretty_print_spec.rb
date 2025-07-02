require 'spec_helper'

describe PgQuery do
  let(:oneline_query) { query.gsub(/\s+/, ' ').gsub('( ', '(').gsub(' )', ')').strip.chomp(';') }

  describe '.deparse' do
    subject { PgQuery.parse(query).deparse(pretty_print: true) }

    context 'SELECT' do
      context 'basic statement' do
        let(:query) {
'SELECT a AS b
FROM x
WHERE
    y = 5
    AND z = y' }

        it { is_expected.to eq query }
      end
    end
  end
end
