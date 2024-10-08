

function handler () {
    EVENT_DATA=$1
    DATA=`ls` # "ls" is in backticks
    RESPONSE="{\"statusCode\": 200, \"body\": \"$DATA\"}"
    echo $RESPONSE
}
