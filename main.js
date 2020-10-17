
import './node_modules/@patternfly/patternfly/patternfly.scss';
import './static/style.scss';

import("./pkg").then(module => {
    module.run_app();
});
