
def handler(event, context):
    event_type = event.get('type')
    match event_type:
        case 'alpha':
            return {
                'stepfn': 'alpha',
                'data': event
            }
        case 'beta':
            return {
                'stepfn': 'beta',
                'data': event
            }
        else:
            return {
                'stepfn': 'default',
                'data': 'event'
            }
