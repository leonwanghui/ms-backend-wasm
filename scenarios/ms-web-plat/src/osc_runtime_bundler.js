const rust =
  import ('../pkg');
import * as YAML from 'yamljs';
import * as request from 'superagent';

let config = {
  user: 'username',
  password: 'password'
};

/**
 * @param {string} operate
 * @param {any} filedata
 * @returns {any}
 */
export function load(operate, filedata) {
  let data = YAML.parse(filedata);
  // Set the operation manually to pass the test case.
  data.metadata.operation = operate;

  rust
    .then(m => {
      let body = m.load_resource_to_js(data);
      switch (body.kind) {
        case 'ComputeResource':
          switch (body.operation) {
            case 'create':
              request.post('/v1alpha/compute_resources')
                .auth(config.user, config.password)
                .send(body.params)
                .set('Content-Type', 'application/json')
                .set('Accept', 'application/json')
                .end(function(err, res) {
                  if (err) throw err;
                  console.log(res.body);
                  document.getElementById('text').innerHTML =
                    JSON.stringify(res.body, null, 2);
                });
              break
            case "delete":
              let opts = body.params.optional;
              request.delete('/v1alpha/compute_resources/' + body.params.name +
                  '?cloud_provider=' + body.params.cloud_provider +
                  '&delete_publicip=' + opts.delete_publicip +
                  '&delete_volume=' + opts.delete_volume)
                .auth(config.user, config.password)
                .set('Accept', 'application/json')
                .end(function(err, res) {
                  if (err) throw err;
                  console.log('delete compute resource {' + body.params.name + '} finished!');
                  document.getElementById('text').innerHTML =
                    'delete compute resource {' + body.params.name + '} finished!';
                });
              break
            default:
              console.error('operation {' + body.operation + '} not supported!');
              document.getElementById('text').innerHTML =
                'operation {' + body.operation + '} not supported!';
          };
          break
        case 'StorageResource':
          switch (body.operation) {
            case 'create':
              request.post('/v1alpha/storage_resources')
                .auth(config.user, config.password)
                .send(body.params)
                .set('Content-Type', 'application/json')
                .set('Accept', 'application/json')
                .end(function(err, res) {
                  if (err) throw err;
                  console.log(res.body);
                  document.getElementById('text').innerHTML =
                    JSON.stringify(res.body, null, 2);
                });
              break
            case "delete":
              request.delete('/v1alpha/storage_resources/' + body.params.name +
                  '?cloud_provider=' + body.params.cloud_provider)
                .auth(config.user, config.password)
                .set('Accept', 'application/json')
                .end(function(err, res) {
                  if (err) throw err;
                  console.log('delete storage resource {' + body.params.name + '} finished!');
                  document.getElementById('text').innerHTML =
                    'delete storage resource {' + body.params.name + '} finished!';
                });
              break
            default:
              console.error('operation {' + body.operation + '} not supported!');
              document.getElementById('text').innerHTML =
                'operation {' + body.operation + '} not supported!';
          };
          break
        case 'NetworkResource':
          switch (body.operation) {
            case 'create':
              request.post('/v1alpha/network_resources')
                .auth(config.user, config.password)
                .send(body.params)
                .set('Content-Type', 'application/json')
                .set('Accept', 'application/json')
                .end(function(err, res) {
                  if (err) throw err;
                  console.log(res.body);
                  document.getElementById('text').innerHTML =
                    JSON.stringify(res.body, null, 2);
                });
              break
            case "delete":
              request.post('/v1alpha/network_resources/' + body.params.name +
                  '?cloud_provider=' + body.params.cloud_provider)
                .auth(config.user, config.password)
                .set('Accept', 'application/json')
                .end(function(err, res) {
                  if (err) throw err;
                  console.log('delete network resource {' + body.params.name + '} finished!');
                  document.getElementById('text').innerHTML =
                    'delete network resource {' + body.params.name + '} finished!';
                });
              break
            default:
              console.error('operation {' + body.operation + '} not supported!');
              document.getElementById('text').innerHTML =
                'operation {' + body.operation + '} not supported!';
          };
          break
        default:
          console.error('resource {' + body.kind + '} not supported!');
          document.getElementById('text').innerHTML =
            'resource {' + body.kind + '} not supported!';
      };
    })
    .catch(console.error);
};
