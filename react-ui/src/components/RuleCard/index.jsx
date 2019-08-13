import React, { Fragment } from 'react';
import { bool, func } from 'prop-types';
import classNames from 'classnames';
import { makeStyles } from '@material-ui/styles';
import Card from '@material-ui/core/Card';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemText from '@material-ui/core/ListItemText';
import Tooltip from '@material-ui/core/Tooltip';
import IconButton from '@material-ui/core/IconButton';
import CardHeader from '@material-ui/core/CardHeader';
import CardContent from '@material-ui/core/CardContent';
import CardActions from '@material-ui/core/CardActions';
import Grid from '@material-ui/core/Grid';
import Avatar from '@material-ui/core/Avatar';
import Divider from '@material-ui/core/Divider';
import Typography from '@material-ui/core/Typography';
import Chip from '@material-ui/core/Chip';
import DeleteIcon from 'mdi-react/DeleteIcon';
import UpdateIcon from 'mdi-react/UpdateIcon';
import PlusCircleIcon from 'mdi-react/PlusCircleIcon';
import HistoryIcon from 'mdi-react/HistoryIcon';
import { formatDistanceStrict } from 'date-fns';
import Button from '../Button';
import DiffRule from '../DiffRule';
import SignoffSummary from '../SignoffSummary';
import { withUser } from '../../utils/AuthContext';
import Link from '../../utils/Link';
import { RULE_DIFF_PROPERTIES } from '../../utils/constants';
import { rule } from '../../utils/prop-types';
import getDiffedProperties from '../../utils/getDiffedProperties';

const useStyles = makeStyles(theme => ({
  root: {
    '& h2, & h4': {
      '& .anchor-link-style': {
        textDecoration: 'none',
        opacity: 0,
        // To prevent the link to get the focus.
        display: 'none',
      },
      '&:hover .anchor-link-style': {
        display: 'inline-block',
        opacity: 1,
        color: theme.palette.text.hint,
        '&:hover': {
          color: theme.palette.text.secondary,
        },
      },
    },
  },
  space: {
    paddingTop: theme.spacing(2),
  },
  cardHeader: {
    paddingBottom: 0,
  },
  cardHeaderAvatar: {
    display: 'flex',
  },
  cardContentRoot: {
    padding: theme.spacing(1),
  },
  deletedText: {
    padding: theme.spacing(1),
  },
  listItem: {
    paddingTop: 0,
    paddingBottom: 0,
  },
  textEllipsis: {
    ...theme.mixins.textEllipsis,
  },
  cardActions: {
    justifyContent: 'flex-end',
  },
  scheduledChangesTitle: {
    padding: `0 ${theme.spacing(1)}px`,
  },
  diff: {
    fontSize: theme.typography.body2.fontSize,
    marginTop: theme.spacing(1),
  },
  chip: {
    height: theme.spacing(3),
  },
  chipIcon: {
    marginLeft: theme.spacing(1),
    marginBottom: 1,
  },
  deleteChip: {
    background: theme.palette.error.main,
    color: theme.palette.error.contrastText,
    '& svg': {
      fill: theme.palette.error.contrastText,
    },
  },
  divider: {
    margin: `${theme.spacing(1)}px`,
  },
  scheduledChangesHeader: {
    display: 'flex',
    justifyContent: 'space-between',
  },
  inline: {
    display: 'inline',
  },
  avatar: {
    height: theme.spacing(4),
    width: theme.spacing(4),
  },
  avatarText: {
    fontSize: 10,
  },
  comment: {
    maxHeight: theme.spacing(10),
    overflowY: 'auto',
  },
  propertyWithScheduledChange: {
    ...theme.mixins.redDot,
  },
  priorityScheduledChange: {
    marginLeft: -10,
    zIndex: 1,
  },
  primaryText: {
    display: 'flex',
    alignItems: 'center',
  },
  link: {
    ...theme.mixins.link,
  },
}));

function RuleCard({
  rule,
  onRuleDelete,
  onSignoff,
  onRevoke,
  user,
  readOnly,
  onAuthorize,
  onUnauthorize,
  ...props
}) {
  const classes = useStyles();
  const requiresSignoff =
    rule.scheduledChange &&
    Object.keys(rule.scheduledChange.required_signoffs).length > 0;
  const getChipIcon = changeType => {
    switch (changeType) {
      case 'delete': {
        return DeleteIcon;
      }

      case 'update': {
        return UpdateIcon;
      }

      case 'insert': {
        return PlusCircleIcon;
      }

      default: {
        return PlusCircleIcon;
      }
    }
  };

  const ChipIcon = getChipIcon(
    rule.scheduledChange && rule.scheduledChange.change_type
  );
  const diffedProperties =
    rule && rule.scheduledChange
      ? getDiffedProperties(RULE_DIFF_PROPERTIES, rule, rule.scheduledChange)
      : [];

  return (
    <Card classes={{ root: classes.root }} spacing={4} {...props}>
      {rule.product && (
        <CardHeader
          classes={{ avatar: classes.cardHeaderAvatar }}
          className={classes.cardHeader}
          avatar={
            Number.isInteger(Number(rule.priority)) && (
              <Fragment>
                <Avatar
                  title="Priority"
                  aria-label="Priority"
                  className={classes.avatar}>
                  <Typography className={classes.avatarText}>
                    {rule.priority}
                  </Typography>
                </Avatar>
                {diffedProperties.includes('priority') &&
                  rule.scheduledChange.change_type === 'update' && (
                    <div
                      className={classNames(
                        classes.propertyWithScheduledChange,
                        classes.priorityScheduledChange
                      )}
                    />
                  )}
              </Fragment>
            )
          }
          title={
            <Typography component="h2" variant="h6">
              {rule.channel
                ? `${rule.product} : ${rule.channel}`
                : rule.product}{' '}
              <a
                href={`#ruleId=${rule.rule_id}`}
                aria-label="Anchor"
                className="anchor-link-style">
                #
              </a>
            </Typography>
          }
          action={
            !readOnly ? (
              <Link to={`/rules/${rule.rule_id}/revisions`}>
                <Tooltip title="Revisions">
                  <IconButton>
                    <HistoryIcon />
                  </IconButton>
                </Tooltip>
              </Link>
            ) : null
          }
        />
      )}
      <CardContent classes={{ root: classes.cardContentRoot }}>
        {(!rule.scheduledChange ||
          rule.scheduledChange.change_type !== 'insert') && (
          <Grid container>
            <Grid item xs={4}>
              <List>
                {rule.mapping && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      title={rule.mapping}
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      secondaryTypographyProps={{
                        className: classes.textEllipsis,
                      }}
                      primary={
                        <Fragment>
                          Mapping
                          {diffedProperties.includes('mapping') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.mapping}
                    />
                  </ListItem>
                )}
                {rule.fallbackMapping && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      title={rule.fallbackMapping}
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      secondaryTypographyProps={{
                        className: classes.textEllipsis,
                      }}
                      primary={
                        <Fragment>
                          Fallback Mapping
                          {diffedProperties.includes('fallbackMapping') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.fallbackMapping}
                    />
                  </ListItem>
                )}
                {Number.isInteger(Number(rule.backgroundRate)) && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Background Rate
                          {diffedProperties.includes('backgroundRate') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.backgroundRate}
                    />
                  </ListItem>
                )}
              </List>
            </Grid>
            <Grid item xs={4}>
              <List>
                {rule.data_version && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Data Version
                          {diffedProperties.includes('data_version') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.data_version}
                    />
                  </ListItem>
                )}
                {Number.isInteger(Number(rule.rule_id)) && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primary="Rule ID"
                      secondary={
                        <Fragment>
                          <Typography
                            component="span"
                            variant="body2"
                            className={classes.inline}
                            color="textPrimary">
                            {rule.rule_id}
                          </Typography>
                          {rule.alias && ` ${rule.alias} (alias)`}
                        </Fragment>
                      }
                    />
                  </ListItem>
                )}
              </List>
            </Grid>
            <Grid item xs={4}>
              <List>
                {rule.version && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Version
                          {diffedProperties.includes('version') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.version}
                    />
                  </ListItem>
                )}
                {rule.buildID && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Build ID
                          {diffedProperties.includes('buildID') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.buildID}
                    />
                  </ListItem>
                )}
                {rule.buildTarget && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Build Target
                          {diffedProperties.includes('buildTarget') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.buildTarget}
                    />
                  </ListItem>
                )}
                {rule.locale && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Locale
                          {diffedProperties.includes('locale') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.locale}
                    />
                  </ListItem>
                )}
                {rule.distribution && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Distribution
                          {diffedProperties.includes('distribution') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.distribution}
                    />
                  </ListItem>
                )}
                {rule.distVersion && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Distribution Version
                          {diffedProperties.includes('distVersion') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.distVersion}
                    />
                  </ListItem>
                )}
                {rule.osVersion && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      title={rule.osVersion}
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      secondaryTypographyProps={{
                        className: classes.textEllipsis,
                      }}
                      primary={
                        <Fragment>
                          OS Version
                          {diffedProperties.includes('osVersion') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.osVersion}
                    />
                  </ListItem>
                )}
                {rule.instructionSet && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Instruction Set
                          {diffedProperties.includes('instructionSet') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.instructionSet}
                    />
                  </ListItem>
                )}
                {rule.memory && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Memory
                          {diffedProperties.includes('memory') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.memory}
                    />
                  </ListItem>
                )}
                {rule.mig64 != null && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          64-bit Migration Opt-in
                          {diffedProperties.includes('mig64') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={String(rule.mig64)}
                    />
                  </ListItem>
                )}
                {rule.jaws != null && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Incompatible JAWS Screen Reader
                          {diffedProperties.includes('jaws') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={String(rule.jaws)}
                    />
                  </ListItem>
                )}
                {rule.headerArchitecture && (
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Header Architecture
                          {diffedProperties.includes('headerArchitecture') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.headerArchitecture}
                    />
                  </ListItem>
                )}
              </List>
            </Grid>
            <Grid item xs={12}>
              {rule.comment && (
                <List>
                  <ListItem className={classes.listItem}>
                    <ListItemText
                      className={classes.comment}
                      primaryTypographyProps={{
                        component: 'div',
                        className: classes.primaryText,
                      }}
                      primary={
                        <Fragment>
                          Comment
                          {diffedProperties.includes('comment') &&
                            rule.scheduledChange.change_type === 'update' && (
                              <span
                                className={classes.propertyWithScheduledChange}
                              />
                            )}
                        </Fragment>
                      }
                      secondary={rule.comment}
                    />
                  </ListItem>
                </List>
              )}
            </Grid>
          </Grid>
        )}
        {rule.scheduledChange && (
          <Fragment>
            {rule.scheduledChange.change_type !== 'insert' && (
              <Divider className={classes.divider} />
            )}
            <div className={classes.scheduledChangesHeader}>
              <Typography
                className={classes.scheduledChangesTitle}
                component="h4"
                variant="subtitle1">
                Scheduled Changes{' '}
                <a
                  href={`#scId=${rule.scheduledChange.sc_id}`}
                  aria-label="Anchor"
                  className="anchor-link-style">
                  #
                </a>
              </Typography>
              <Chip
                className={classNames(classes.chip, {
                  [classes.deleteChip]:
                    rule.scheduledChange.change_type === 'delete',
                })}
                icon={<ChipIcon className={classes.chipIcon} size={16} />}
                label={`${formatDistanceStrict(
                  rule.scheduledChange.when,
                  new Date(),
                  { addSuffix: true }
                )} (${rule.scheduledChange.change_type})`}
              />
            </div>
            {rule.scheduledChange.change_type === 'delete' ? (
              <Typography
                className={classes.deletedText}
                variant="body2"
                color="textSecondary">
                All properties will be deleted
              </Typography>
            ) : (
              <DiffRule firstRule={rule} secondRule={rule.scheduledChange} />
            )}
          </Fragment>
        )}
        {!readOnly && requiresSignoff && (
          <Fragment>
            <SignoffSummary
              requiredSignoffs={rule.scheduledChange.required_signoffs}
              signoffs={rule.scheduledChange.signoffs}
              className={classes.space}
            />
          </Fragment>
        )}
      </CardContent>
      {!readOnly && (
        <CardActions className={classes.cardActions}>
          {user ? (
            <Link
              className={classes.link}
              to={{
                pathname: '/rules/create',
                state: {
                  rule,
                },
              }}>
              <Button color="secondary">Duplicate</Button>
            </Link>
          ) : (
            <Button color="secondary" disabled>
              Duplicate
            </Button>
          )}
          {user ? (
            <Link
              className={classes.link}
              to={
                rule.rule_id
                  ? `/rules/${rule.rule_id}`
                  : `/rules/create/${rule.scheduledChange.sc_id}`
              }>
              <Button color="secondary">Update</Button>
            </Link>
          ) : (
            <Button color="secondary" disabled>
              Update
            </Button>
          )}
          <Button
            color="secondary"
            disabled={!user}
            onClick={() => onRuleDelete(rule)}>
            Delete
          </Button>
          {requiresSignoff &&
            (user && user.email in rule.scheduledChange.signoffs ? (
              <Button color="secondary" disabled={!user} onClick={onRevoke}>
                Revoke Signoff
              </Button>
            ) : (
              <Button color="secondary" disabled={!user} onClick={onSignoff}>
                Signoff
              </Button>
            ))}
        </CardActions>
      )}
    </Card>
  );
}

RuleCard.propTypes = {
  rule: rule.isRequired,
  onRuleDelete: func,
  // If true, the card will hide all buttons.
  readOnly: bool,
  // These are required if readOnly is false
  onSignoff: func,
  onRevoke: func,
};

RuleCard.defaultProps = {
  onRuleDelete: Function.prototype,
  readOnly: false,
};

export default withUser(RuleCard);