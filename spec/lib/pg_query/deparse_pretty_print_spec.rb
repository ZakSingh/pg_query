require 'spec_helper'

describe PgQuery do
  describe '.deparse' do
    subject { PgQuery.parse(query).deparse(pretty_print: true) }

    context 'SELECT' do
      context 'basic statement' do
        let(:query) do
          <<~Q
            SELECT a AS b
            FROM x
            WHERE
                y = 5
                AND z = y
          Q
        end

        it { is_expected.to eq query.strip }
      end
    end
  end
end
